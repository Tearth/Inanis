use crate::{board::*, common::*, movescan::*};
use std::mem::MaybeUninit;

pub fn run(depth: i32) -> u64 {
    let mut board = Bitboard::new_default();
    let count = run_internal::<WHITE>(depth, &mut board);

    count
}

pub fn run_divided(depth: i32, premade_moves: &[&str]) -> Vec<(String, u64)> {
    let mut board = Bitboard::new_default();

    for premade_move in premade_moves {
        let parsed_move = Move::from_text(premade_move.trim(), &board);
        board.make_move(&parsed_move);
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = match board.color_to_move {
        WHITE => board.get_moves::<WHITE>(&mut moves),
        BLACK => board.get_moves::<BLACK>(&mut moves),
        _ => panic!("Invalid value: board.color_to_move={}", board.color_to_move),
    };

    let mut result = Vec::<(String, u64)>::new();
    for r#move in &moves[0..moves_count] {
        board.make_move(r#move);

        let moves_count = match board.color_to_move {
            WHITE => run_internal::<WHITE>(depth - 1, &mut board),
            BLACK => run_internal::<BLACK>(depth - 1, &mut board),
            _ => panic!("Invalid value: board.color_to_move={}", board.color_to_move),
        };

        result.push((r#move.to_text(), moves_count));
        board.undo_move(r#move);
    }

    result
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
