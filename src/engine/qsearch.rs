use super::common::*;
use super::context::SearchContext;
use crate::evaluation::material;
use crate::state::common::*;
use crate::state::movescan::Move;
use crate::state::movescan::MoveFlags;
use std::mem::MaybeUninit;

macro_rules! run_qsearch {
    ($color:expr, $context:expr, $depth:expr, $ply:expr, $alpha:expr, $beta:expr, $invert:expr) => {
        match $invert {
            true => match $color {
                WHITE => run::<BLACK>($context, $depth, $ply, $alpha, $beta),
                BLACK => run::<WHITE>($context, $depth, $ply, $alpha, $beta),
                _ => panic!("Invalid value: $color={}", $color),
            },
            false => match $color {
                WHITE => run::<WHITE>($context, $depth, $ply, $alpha, $beta),
                BLACK => run::<BLACK>($context, $depth, $ply, $alpha, $beta),
                _ => panic!("Invalid value: $color={}", $color),
            },
        }
    };
}

pub fn run<const COLOR: u8>(context: &mut SearchContext, depth: i32, ply: u16, mut alpha: i16, beta: i16) -> i16 {
    context.statistics.q_nodes_count += 1;

    if context.board.pieces[COLOR as usize][KING as usize] == 0 {
        context.statistics.q_leafs_count += 1;
        return -CHECKMATE_SCORE + (ply as i16);
    }

    let stand_pat = -((COLOR as i16) * 2 - 1) * context.board.evaluate();
    if stand_pat >= beta {
        context.statistics.q_leafs_count += 1;
        context.statistics.q_beta_cutoffs += 1;
        return beta;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut move_scores: [i16; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves::<COLOR>(&mut moves);

    assign_move_scores(context, &moves, &mut move_scores, moves_count);

    let mut found = false;
    for move_index in 0..moves_count {
        sort_next_move(&mut moves, &mut move_scores, move_index, moves_count);

        let r#move = moves[move_index];
        if r#move.get_flags() != MoveFlags::CAPTURE {
            continue;
        }

        found = true;

        context.board.make_move::<COLOR>(&r#move);
        let score = -run_qsearch!(COLOR, context, depth - 1, ply + 1, -beta, -alpha, true);
        context.board.undo_move::<COLOR>(&r#move);

        if score > alpha {
            alpha = score;

            if alpha >= beta {
                context.statistics.q_beta_cutoffs += 1;
                if move_index == 0 {
                    context.statistics.q_perfect_cutoffs += 1;
                } else {
                    context.statistics.q_non_perfect_cutoffs += 1;
                }

                break;
            }
        }
    }

    if !found {
        context.statistics.q_leafs_count += 1;
    }

    alpha
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

        let attacking_piece_value = material::PIECE_VALUE[attacking_piece as usize];
        let captured_piece_value = material::PIECE_VALUE[captured_piece as usize];

        move_scores[move_index] = captured_piece_value - attacking_piece_value;
    }
}
