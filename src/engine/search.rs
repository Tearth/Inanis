use super::context::SearchContext;
use super::context::SearchResult;
use super::history::HistoryTable;
use super::killers::KillersTable;
use super::qsearch;
use super::*;
use crate::cache::pawns::PawnsHashTable;
use crate::cache::search::TranspositionTable;
use crate::cache::search::TranspositionTableScoreType;
use crate::state::board::Bitboard;
use crate::state::movescan::Move;
use crate::state::movescan::MoveFlags;
use crate::state::*;
use chrono::Utc;
use std::mem::MaybeUninit;

pub fn run_fixed_depth(board: &mut Bitboard, depth: i32) -> SearchResult {
    let transposition_table_size = 32 * 1024 * 1024;
    let mut transposition_table = TranspositionTable::new(transposition_table_size);
    let mut pawns_table = PawnsHashTable::new(4 * 1024 * 1024);
    let mut killers_table = KillersTable::new();
    let mut history_table = HistoryTable::new();

    let mut context = SearchContext::new(
        board,
        0,
        0,
        &mut transposition_table,
        &mut pawns_table,
        &mut killers_table,
        &mut history_table,
    );
    let mut best_move = Move::new_empty();
    let mut best_score = 0;

    context.deadline = u32::MAX;

    let search_time_start = Utc::now();
    for depth in 1..=depth {
        let score = run::<true>(&mut context, depth, 0, -32000, 32000, true);
        let r#move = context.transposition_table.get(context.board.hash, 0).best_move;

        best_score = score;
        best_move = r#move;
    }

    let time = (Utc::now() - search_time_start).num_milliseconds() as u64;
    SearchResult::new(time, depth, best_score, best_move, context.statistics)
}

pub fn run<const PV: bool>(context: &mut SearchContext, depth: i32, ply: u16, mut alpha: i16, mut beta: i16, allow_null_move: bool) -> i16 {
    // Check every 100 000 node
    if context.statistics.nodes_count % 100_000 == 0 {
        if (Utc::now() - context.search_time_start).num_milliseconds() >= context.deadline as i64 {
            context.aborted = true;
            return 0;
        }
    }

    context.statistics.nodes_count += 1;

    if context.board.pieces[context.board.active_color as usize][KING as usize] == 0 {
        context.statistics.leafs_count += 1;
        return -CHECKMATE_SCORE + (ply as i16);
    }

    if context.board.is_threefold_repetition_draw() || context.board.is_fifty_move_rule_draw() {
        context.statistics.leafs_count += 1;
        return 0;
    }

    if depth <= 0 {
        context.statistics.leafs_count += 1;
        return qsearch::run(context, depth, ply, alpha, beta);
    }

    let original_alpha = alpha;
    let mut tt_entry_found = false;
    let tt_entry = context.transposition_table.get(context.board.hash, ply);

    if tt_entry.key == (context.board.hash >> 32) as u32 {
        context.statistics.tt_hits += 1;

        if tt_entry.depth >= depth as i8 {
            tt_entry_found = true;
            match tt_entry.score_type {
                TranspositionTableScoreType::ALPHA_SCORE => {
                    if tt_entry.score < beta {
                        beta = tt_entry.score;
                    }
                }
                TranspositionTableScoreType::BETA_SCORE => {
                    if tt_entry.score > alpha {
                        alpha = tt_entry.score;
                    }
                }
                _ => return tt_entry.score,
            }

            if alpha >= beta {
                context.statistics.beta_cutoffs += 1;
                return tt_entry.score;
            }
        }
    } else {
        context.statistics.tt_misses += 1;
    }

    // Null-move pruning
    if !PV
        && allow_null_move
        && depth > 3
        && context.board.get_game_phase() > 0.15
        && !context.board.is_king_checked(context.board.active_color)
    {
        let r = if depth > 6 { 3 } else { 2 };
        context.statistics.null_window_searches += 1;

        context.board.make_null_move();
        let score = -run::<false>(context, depth - r - 1, ply + 1, -beta, -beta + 1, false);
        context.board.undo_null_move();

        if score >= beta {
            context.statistics.null_window_accepted += 1;
            return score;
        } else {
            context.statistics.null_window_rejected += 1;
        }
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut move_scores: [i16; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves(&mut moves);

    assign_move_scores(context, &moves, &mut move_scores, moves_count, tt_entry.best_move, ply);

    let mut best_move = Move::new_empty();
    let mut best_score = i16::MIN;

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
                if r#move.get_flags() == MoveFlags::QUIET || r#move.get_flags() == MoveFlags::DOUBLE_PUSH {
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

    if context.aborted {
        return -1;
    }

    if best_score == -(-CHECKMATE_SCORE + (ply as i16) + 1) {
        context.statistics.leafs_count += 1;
        return best_score;
    }

    if best_score == -CHECKMATE_SCORE + (ply as i16) + 2 && !context.board.is_king_checked(context.board.active_color) {
        context.statistics.leafs_count += 1;
        return 0;
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
        context.statistics.tt_added_entries += 1;
    }

    best_score
}

fn assign_move_scores(context: &SearchContext, moves: &[Move], move_scores: &mut [i16], moves_count: usize, tt_move: Move, ply: u16) {
    for move_index in 0..moves_count {
        let r#move = moves[move_index];

        if r#move == tt_move {
            move_scores[move_index] = 10000;
            continue;
        }

        if context.killers_table.exists(context.board.active_color, ply, r#move) {
            move_scores[move_index] = 120;
            continue;
        }

        if r#move.get_flags() == MoveFlags::CAPTURE {
            let field = r#move.get_to();
            let attacking_piece = context.board.get_piece(r#move.get_from());
            let captured_piece = context.board.get_piece(r#move.get_to());
            let attackers = context.board.get_attacking_pieces(context.board.active_color ^ 1, field);
            let defenders = context.board.get_attacking_pieces(context.board.active_color, field);

            move_scores[move_index] = (see::get(attacking_piece, captured_piece, attackers, defenders) as i16) * 100;
            continue;
        }

        move_scores[move_index] = context.history_table.get(r#move.get_from(), r#move.get_to(), 90) as i16;
    }
}
