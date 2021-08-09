use super::common::*;
use super::context::SearchContext;
use super::context::SearchResult;
use super::qsearch;
use crate::board::common::*;
use crate::board::movescan::Move;
use crate::board::movescan::MoveFlags;
use crate::board::repr::Bitboard;
use crate::evaluation::values;
use chrono::Utc;
use std::mem::MaybeUninit;

#[macro_export]
macro_rules! run_search {
    ($color:expr, $context:expr, $depth:expr, $alpha:expr, $beta:expr, $invert:expr) => {
        match $invert {
            true => match $color {
                crate::board::common::WHITE => {
                    crate::engine::search::run::<{ crate::board::common::BLACK }>($context, $depth, $alpha, $beta)
                }
                crate::board::common::BLACK => {
                    crate::engine::search::run::<{ crate::board::common::WHITE }>($context, $depth, $alpha, $beta)
                }
                _ => panic!("Invalid value: $color={}", $color),
            },
            false => match $color {
                crate::board::common::WHITE => {
                    crate::engine::search::run::<{ crate::board::common::WHITE }>($context, $depth, $alpha, $beta)
                }
                crate::board::common::BLACK => {
                    crate::engine::search::run::<{ crate::board::common::BLACK }>($context, $depth, $alpha, $beta)
                }
                _ => panic!("Invalid value: $color={}", $color),
            },
        }
    };
}

pub fn run_fixed_depth(board: &mut Bitboard, depth: i32) -> SearchResult {
    let mut context = SearchContext::new(board, 0, 0);
    let mut best_move = Move::new(0, 0, MoveFlags::QUIET);
    let mut best_score = 0;

    let search_time_start = Utc::now();
    for depth in 1..=depth {
        let (score, r#move) = run_search!(context.board.active_color, &mut context, depth, -32000, 32000, false);

        best_score = score;
        best_move = r#move;
    }

    let time = (Utc::now() - search_time_start).num_milliseconds() as u64;
    SearchResult::new(time, depth, best_score, best_move, context.statistics)
}

pub fn run<const COLOR: u8>(context: &mut SearchContext, depth: i32, mut alpha: i16, beta: i16) -> (i16, Move) {
    context.statistics.nodes_count += 1;

    if context.board.pieces[COLOR as usize][KING as usize] == 0 {
        context.statistics.leafs_count += 1;
        return (-31900 - (depth as i16), Move::new(0, 0, MoveFlags::QUIET));
    }

    if depth <= 0 {
        context.statistics.leafs_count += 1;
        return (
            qsearch::run::<COLOR>(context, depth, alpha, beta),
            Move::new(0, 0, MoveFlags::QUIET),
        );
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut move_scores: [i16; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves::<COLOR>(&mut moves);

    assign_move_scores(context, &moves, &mut move_scores, moves_count);

    let mut best_move = Move::new(0, 0, MoveFlags::QUIET);
    for move_index in 0..moves_count {
        sort_next_move(&mut moves, &mut move_scores, move_index, moves_count);

        let r#move = moves[move_index];

        context.board.make_move::<COLOR>(&r#move);
        let (search_score, _) = run_search!(COLOR, context, depth - 1, -beta, -alpha, true);
        let score = -search_score;
        context.board.undo_move::<COLOR>(&r#move);

        if score > alpha {
            alpha = score;
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

    (alpha, best_move)
}

fn assign_move_scores(context: &SearchContext, moves: &[Move], move_scores: &mut [i16], moves_count: usize) {
    for move_index in 0..moves_count {
        let r#move = moves[move_index];

        if r#move.get_flags() != MoveFlags::CAPTURE {
            move_scores[move_index] = 0;
            continue;
        }

        let attacking_piece = context.board.get_piece(r#move.get_from());
        let captured_piece = context.board.get_piece(r#move.get_to());

        let attacking_piece_value = values::PIECE_VALUE[attacking_piece as usize];
        let captured_piece_value = values::PIECE_VALUE[captured_piece as usize];

        move_scores[move_index] = captured_piece_value - attacking_piece_value;
    }
}
