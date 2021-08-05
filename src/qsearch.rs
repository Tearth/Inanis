use crate::common::*;
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
    if depth < -15 {
        return stand_pat;
    }

    if stand_pat >= beta {
        return beta;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves::<COLOR>(&mut moves);

    for r#move in &moves[0..moves_count] {
        if r#move.get_flags() != MoveFlags::CAPTURE {
            continue;
        }

        context.board.make_move::<COLOR>(r#move);
        let score = -run_internal!(COLOR, context, depth - 1, -beta, -alpha, true);
        context.board.undo_move::<COLOR>(r#move);

        if score > alpha {
            alpha = score;

            if alpha >= beta {
                return beta;
            }
        }
    }

    alpha
}
