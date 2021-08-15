use super::common::*;
use super::context::SearchContext;
use super::context::SearchResult;
use super::qsearch;
use crate::cache::search::TranspositionTable;
use crate::cache::search::TranspositionTableScoreType;
use crate::evaluation::material;
use crate::state::board::Bitboard;
use crate::state::common::*;
use crate::state::movescan::Move;
use crate::state::movescan::MoveFlags;
use chrono::Utc;
use std::mem::MaybeUninit;
use std::sync::Arc;

#[macro_export]
macro_rules! run_search {
    ($color:expr, $context:expr, $depth:expr, $ply:expr, $alpha:expr, $beta:expr, $invert:expr) => {
        match $invert {
            true => match $color {
                crate::state::common::WHITE => {
                    crate::engine::search::run::<{ crate::state::common::BLACK }>($context, $depth, $ply, $alpha, $beta)
                }
                crate::state::common::BLACK => {
                    crate::engine::search::run::<{ crate::state::common::WHITE }>($context, $depth, $ply, $alpha, $beta)
                }
                _ => panic!("Invalid value: $color={}", $color),
            },
            false => match $color {
                crate::state::common::WHITE => {
                    crate::engine::search::run::<{ crate::state::common::WHITE }>($context, $depth, $ply, $alpha, $beta)
                }
                crate::state::common::BLACK => {
                    crate::engine::search::run::<{ crate::state::common::BLACK }>($context, $depth, $ply, $alpha, $beta)
                }
                _ => panic!("Invalid value: $color={}", $color),
            },
        }
    };
}

pub fn run_fixed_depth(board: &mut Bitboard, depth: i32) -> SearchResult {
    let transposition_table_size_mb = 32 * 1024 * 1024;
    let mut transposition_table = TranspositionTable::new(transposition_table_size_mb);

    let mut context = SearchContext::new(board, 0, 0, &mut transposition_table);
    let mut best_move = Move::new_empty();
    let mut best_score = 0;

    context.deadline = u32::MAX;

    let search_time_start = Utc::now();
    for depth in 1..=depth {
        let score = run_search!(context.board.active_color, &mut context, depth, 0, -32000, 32000, false);
        let r#move = context.transposition_table.get(context.board.hash, 0).best_move;

        best_score = score;
        best_move = r#move;
    }

    let time = (Utc::now() - search_time_start).num_milliseconds() as u64;
    SearchResult::new(time, depth, best_score, best_move, context.statistics)
}

pub fn run<const COLOR: u8>(context: &mut SearchContext, depth: i32, ply: u16, mut alpha: i16, mut beta: i16) -> i16 {
    // Check every 100 000 node
    if context.statistics.nodes_count % 100_000 == 0 {
        if (Utc::now() - context.search_time_start).num_milliseconds() >= context.deadline as i64 {
            context.aborted = true;
            return 0;
        }
    }

    context.statistics.nodes_count += 1;

    if context.board.pieces[COLOR as usize][KING as usize] == 0 {
        context.statistics.leafs_count += 1;
        return -CHECKMATE_SCORE + (ply as i16);
    }

    if context.board.is_threefold_repetition_draw() || context.board.is_fifty_move_rule_draw() {
        context.statistics.leafs_count += 1;
        return 0;
    }

    if depth <= 0 {
        context.statistics.leafs_count += 1;
        return qsearch::run::<COLOR>(context, depth, ply, alpha, beta);
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

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut move_scores: [i16; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves::<COLOR>(&mut moves);

    assign_move_scores(context, &moves, &mut move_scores, moves_count, tt_entry.best_move);

    let mut best_move = Move::new_empty();
    let mut best_score = i16::MIN;

    for move_index in 0..moves_count {
        sort_next_move(&mut moves, &mut move_scores, move_index, moves_count);

        let r#move = moves[move_index];

        context.board.make_move::<COLOR>(&r#move);
        let score = -run_search!(COLOR, context, depth - 1, ply + 1, -beta, -alpha, true);
        context.board.undo_move::<COLOR>(&r#move);

        if score > best_score {
            best_score = score;
        }

        if best_score > alpha {
            alpha = best_score;
            best_move = r#move;

            if alpha >= beta {
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

    if best_score == -CHECKMATE_SCORE + (ply as i16) + 2 && !context.board.is_king_checked(COLOR) {
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

fn assign_move_scores(context: &SearchContext, moves: &[Move], move_scores: &mut [i16], moves_count: usize, tt_move: Move) {
    for move_index in 0..moves_count {
        let r#move = moves[move_index];

        if r#move == tt_move {
            move_scores[move_index] = 10000;
            continue;
        }

        if r#move.get_flags() != MoveFlags::CAPTURE {
            move_scores[move_index] = 0;
            continue;
        }

        let attacking_piece = context.board.get_piece(r#move.get_from());
        let captured_piece = context.board.get_piece(r#move.get_to());

        let attacking_piece_value = material::PIECE_VALUE[attacking_piece as usize];
        let captured_piece_value = material::PIECE_VALUE[captured_piece as usize];

        move_scores[move_index] = captured_piece_value - attacking_piece_value;
    }
}
