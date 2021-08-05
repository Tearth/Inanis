use crate::common::*;
use crate::evaluation;
use crate::movescan::Move;
use crate::movescan::MoveFlags;
use crate::search::SearchContext;
use std::mem::MaybeUninit;

macro_rules! run_internal {
    ($color:expr, $context:expr, $depth:expr, $alpha:expr, $beta:expr, $invert:expr) => {
        match $invert {
            true => match $color {
                WHITE => run::<BLACK>($context, $depth, $alpha, $beta),
                BLACK => run::<WHITE>($context, $depth, $alpha, $beta),
                _ => panic!("Invalid value: $color={}", $color),
            },
            false => match $color {
                WHITE => run::<WHITE>($context, $depth, $alpha, $beta),
                BLACK => run::<BLACK>($context, $depth, $alpha, $beta),
                _ => panic!("Invalid value: $color={}", $color),
            },
        }
    };
}

pub fn run<const COLOR: u8>(context: &mut SearchContext, depth: i32, mut alpha: i16, beta: i16) -> i16 {
    if context.board.pieces[COLOR as usize][KING as usize] == 0 {
        return -32000;
    }

    let stand_pat = ((COLOR as i16) * 2 - 1) * context.board.evaluate();
    if stand_pat >= beta {
        return beta;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut move_scores: [i16; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves::<COLOR>(&mut moves);

    assign_move_scores(context, &moves, &mut move_scores, moves_count);

    for move_index in 0..moves_count {
        let r#move = get_next_move(&moves, &move_scores, move_index, moves_count);

        if r#move.get_flags() != MoveFlags::CAPTURE {
            continue;
        }

        context.board.make_move::<COLOR>(&r#move);
        let score = -run_internal!(COLOR, context, depth - 1, -beta, -alpha, true);
        context.board.undo_move::<COLOR>(&r#move);

        if score > alpha {
            alpha = score;

            if alpha >= beta {
                return beta;
            }
        }
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

        let attacking_piece_value = evaluation::get_piece_value(attacking_piece);
        let captured_piece_value = evaluation::get_piece_value(captured_piece);

        move_scores[move_index] = captured_piece_value - attacking_piece_value;
    }
}

fn get_next_move(moves: &[Move], move_scores: &[i16], start_index: usize, moves_count: usize) -> Move {
    let mut best_score = move_scores[start_index];
    let mut best_index = start_index;

    for index in (start_index + 1)..moves_count {
        if move_scores[index] > best_score {
            best_score = move_scores[index];
            best_index = index;
        }
    }

    moves[best_index]
}
