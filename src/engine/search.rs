use super::context::SearchContext;
use super::qsearch;
use super::*;
use crate::cache::search::TranspositionTableScoreType;
use crate::state::movescan::Move;
use crate::state::movescan::MoveFlags;
use crate::state::*;
use chrono::Utc;
use std::mem::MaybeUninit;

pub const NULL_MOVE_MIN_DEPTH: i8 = 4;
pub const NULL_MOVE_R_CHANGE_DEPTH: i8 = 6;
pub const NULL_MOVE_MIN_GAME_PHASE: f32 = 0.15;
pub const NULL_MOVE_SMALL_R: i8 = 2;
pub const NULL_MOVE_BIG_R: i8 = 3;

pub const MOVE_ORDERING_HAS_MOVE: i16 = 10000;
pub const MOVE_ORDERING_KILLER_MOVE: i16 = 120;
pub const MOVE_ORDERING_HISTORY_MOVE: u8 = 90;

pub fn run<const PV: bool>(context: &mut SearchContext, depth: i8, ply: u16, mut alpha: i16, mut beta: i16, allow_null_move: bool) -> i16 {
    if context.abort_token.aborted {
        return INVALID_SCORE;
    }

    // Check deadline every 100 000 node (only if we don't search to the specified depth or nodes count)
    if context.forced_depth == 0 && context.max_nodes_count == 0 && context.statistics.nodes_count % 100_000 == 0 {
        if (Utc::now() - context.search_time_start).num_milliseconds() >= context.deadline as i64 {
            context.abort_token.aborted = true;
            return INVALID_SCORE;
        }
    }

    // Check nodes count (only in PV nodes, doesn't need to be very accurate)
    if PV && context.max_nodes_count != 0 {
        if context.statistics.nodes_count + context.statistics.q_nodes_count >= context.max_nodes_count {
            context.abort_token.aborted = true;
            return INVALID_SCORE;
        }
    }

    context.statistics.nodes_count += 1;

    if context.board.pieces[context.board.active_color as usize][KING as usize] == 0 {
        context.statistics.leafs_count += 1;
        return -CHECKMATE_SCORE + (ply as i16);
    }

    if context.board.is_threefold_repetition_draw() || context.board.is_fifty_move_rule_draw() {
        context.statistics.leafs_count += 1;
        return DRAW_SCORE;
    }

    if depth <= 0 {
        context.statistics.leafs_count += 1;
        return qsearch::run(context, depth, ply, alpha, beta);
    }

    let original_alpha = alpha;
    let mut tt_entry_found = false;
    let mut hash_move = Default::default();
    let mut collision = false;

    match context.transposition_table.get(context.board.hash, ply, &mut collision) {
        Some(entry) => {
            hash_move = entry.best_move;
            context.statistics.tt_hits += 1;

            if entry.depth >= depth as i8 {
                tt_entry_found = true;
                match entry.score_type {
                    TranspositionTableScoreType::ALPHA_SCORE => {
                        if entry.score < beta {
                            beta = entry.score;
                        }
                    }
                    TranspositionTableScoreType::BETA_SCORE => {
                        if entry.score > alpha {
                            alpha = entry.score;
                        }
                    }
                    _ => {
                        context.statistics.leafs_count += 1;
                        return entry.score;
                    }
                }

                if alpha >= beta {
                    context.statistics.leafs_count += 1;
                    context.statistics.beta_cutoffs += 1;
                    return entry.score;
                }
            }
        }
        None => {
            if collision {
                context.statistics.tt_collisions += 1;
            }

            context.statistics.tt_misses += 1;
        }
    };

    if null_move_can_be_applied::<PV>(context, depth, allow_null_move) {
        let r = if depth <= NULL_MOVE_R_CHANGE_DEPTH {
            NULL_MOVE_SMALL_R
        } else {
            NULL_MOVE_BIG_R
        };
        context.statistics.null_move_searches += 1;

        context.board.make_null_move();
        let score = -run::<false>(context, depth - r - 1, ply + 1, -beta, -beta + 1, false);
        context.board.undo_null_move();

        if score >= beta {
            context.statistics.leafs_count += 1;
            context.statistics.null_move_accepted += 1;
            return score;
        } else {
            context.statistics.null_move_rejected += 1;
        }
    }

    let mut best_score = i16::MIN;
    let mut best_move = Default::default();
    let mut moves: [Move; MAX_MOVES_COUNT] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut move_scores: [i16; MAX_MOVES_COUNT] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves(&mut moves);

    assign_move_scores(context, &moves, &mut move_scores, moves_count, hash_move, ply);

    for move_index in 0..moves_count {
        sort_next_move(&mut moves, &mut move_scores, move_index, moves_count);

        let r#move = moves[move_index];
        context.board.make_move(&r#move);

        let score = if PV {
            if move_index == 0 {
                context.statistics.pvs_full_window_searches += 1;
                -run::<true>(context, depth - 1, ply + 1, -beta, -alpha, allow_null_move)
            } else {
                let zero_window_score = -run::<false>(context, depth - 1, ply + 1, -alpha - 1, -alpha, allow_null_move);
                context.statistics.pvs_zero_window_searches += 1;

                if zero_window_score > alpha {
                    context.statistics.pvs_rejected_searches += 1;
                    -run::<true>(context, depth - 1, ply + 1, -beta, -alpha, allow_null_move)
                } else {
                    zero_window_score
                }
            }
        } else {
            -run::<false>(context, depth - 1, ply + 1, -beta, -alpha, allow_null_move)
        };

        context.board.undo_move(&r#move);

        if score > best_score {
            best_score = score;
        }

        if best_score > alpha {
            alpha = best_score;
            best_move = r#move;

            if alpha >= beta {
                if r#move.is_quiet() {
                    context.killers_table.add(context.board.active_color, ply, r#move);
                    context.history_table.add(r#move.get_from(), r#move.get_to(), depth as u8);
                }

                context.statistics.beta_cutoffs += 1;
                if move_index == 0 {
                    context.statistics.perfect_cutoffs += 1;
                } else {
                    context.statistics.non_perfect_cutoffs += 1;
                }

                break;
            }
        }
    }

    if context.abort_token.aborted {
        return INVALID_SCORE;
    }

    if best_score == -(-CHECKMATE_SCORE + (ply as i16) + 1) {
        context.statistics.leafs_count += 1;
        return best_score;
    }

    if best_score == -CHECKMATE_SCORE + (ply as i16) + 2 && !context.board.is_king_checked(context.board.active_color) {
        context.statistics.leafs_count += 1;
        return DRAW_SCORE;
    }

    if !tt_entry_found || alpha != original_alpha {
        let score_type = if alpha <= original_alpha {
            TranspositionTableScoreType::ALPHA_SCORE
        } else if alpha >= beta {
            TranspositionTableScoreType::BETA_SCORE
        } else {
            TranspositionTableScoreType::EXACT_SCORE
        };

        context
            .transposition_table
            .add(context.board.hash, alpha, best_move, depth as i8, ply, score_type);
        context.statistics.tt_added += 1;
    }

    best_score
}

fn assign_move_scores(context: &SearchContext, moves: &[Move], move_scores: &mut [i16], moves_count: usize, tt_move: Move, ply: u16) {
    for move_index in 0..moves_count {
        let r#move = moves[move_index];

        if r#move == tt_move {
            move_scores[move_index] = MOVE_ORDERING_HAS_MOVE;
            continue;
        }

        if context.killers_table.exists(context.board.active_color, ply, r#move) {
            move_scores[move_index] = MOVE_ORDERING_KILLER_MOVE;
            continue;
        }

        if r#move.is_capture() {
            let field = r#move.get_to();
            let attacking_piece = context.board.get_piece(r#move.get_from());
            let captured_piece = context.board.get_piece(r#move.get_to());
            let attackers = context.board.get_attacking_pieces(context.board.active_color ^ 1, field);
            let defenders = context.board.get_attacking_pieces(context.board.active_color, field);

            move_scores[move_index] = (see::get(attacking_piece, captured_piece, attackers, defenders) as i16) * 100;
            continue;
        }

        move_scores[move_index] = context.history_table.get(r#move.get_from(), r#move.get_to(), MOVE_ORDERING_HISTORY_MOVE) as i16;
    }
}

fn null_move_can_be_applied<const PV: bool>(context: &mut SearchContext, depth: i8, allow_null_move: bool) -> bool {
    !PV && allow_null_move
        && depth >= NULL_MOVE_MIN_DEPTH
        && context.board.get_game_phase() > NULL_MOVE_MIN_GAME_PHASE
        && !context.board.is_king_checked(context.board.active_color)
}
