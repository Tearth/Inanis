use crate::board::Bitboard;
use crate::common::*;
use crate::movescan::Move;
use crate::movescan::MoveFlags;
use std::mem::MaybeUninit;

macro_rules! run_internal {
    ($color:expr, $context:expr, $depth:expr, $invert:expr) => {
        match $invert {
            true => match $color {
                WHITE => run_internal::<BLACK>($context, $depth),
                BLACK => run_internal::<WHITE>($context, $depth),
                _ => panic!("Invalid value: $color={}", $color),
            },
            false => match $color {
                WHITE => run_internal::<WHITE>($context, $depth),
                BLACK => run_internal::<BLACK>($context, $depth),
                _ => panic!("Invalid value: $color={}", $color),
            },
        }
    };
}

pub struct SearchContext<'a> {
    pub board: &'a mut Bitboard,
}

pub fn run(board: &mut Bitboard, depth: i32) -> Move {
    let mut context = SearchContext { board };
    run_internal!(context.board.active_color, &mut context, depth, false).1
}

pub fn run_internal<const COLOR: u8>(context: &mut SearchContext, depth: i32) -> (i16, Move) {
    if context.board.pieces[COLOR as usize][KING as usize] == 0 {
        return (-32000, Move::new(0, 0, MoveFlags::QUIET));
    }

    if depth <= 0 {
        return (
            ((COLOR as i16) * 2 - 1) * context.board.evaluate(),
            Move::new(0, 0, MoveFlags::QUIET),
        );
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves::<COLOR>(&mut moves);

    let mut score = i16::MIN;
    let mut best_move = Move::new(0, 0, MoveFlags::QUIET);

    for r#move in &moves[0..moves_count] {
        context.board.make_move::<COLOR>(r#move);

        if !context.board.is_king_checked(COLOR) {
            let (search_score, _) = run_internal!(COLOR, context, depth - 1, true);

            if search_score > score {
                score = search_score;
                best_move = *r#move;
            }
        }

        context.board.undo_move::<COLOR>(r#move);
    }

    (score, best_move)
}
