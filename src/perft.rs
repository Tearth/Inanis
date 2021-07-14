use crate::{board::*, common::*, movescan::*};
use std::mem::MaybeUninit;

pub fn run(depth: i32) -> u64 {
    let mut board = Bitboard::new(false);
    let count = run_internal::<WHITE>(depth, &mut board);

    count
}

fn run_internal<const COLOR: u8>(depth: i32, board: &mut Bitboard) -> u64 {
    if depth <= 0 {
        return 1;
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_moves::<COLOR>(&mut moves);

    let mut count = 0;
    for r#move in &moves[0..moves_count] {
        board.make_move(r#move);

        if !board.is_king_checked::<COLOR>() {
            count += match COLOR {
                WHITE => run_internal::<BLACK>(depth - 1, board),
                BLACK => run_internal::<WHITE>(depth - 1, board),
                _ => panic!("Invalid value: COLOR={}", COLOR),
            };
        }

        board.undo_move(r#move);
    }

    count
}
