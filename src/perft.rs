use crate::board::Bitboard;
use crate::common::*;
use crate::movescan::Move;
use std::mem::MaybeUninit;

pub fn run(depth: i32) -> u64 {
    let mut board = Bitboard::new_default();
    let count = run_internal::<WHITE, BLACK>(depth, &mut board);

    count
}

pub fn run_divided(depth: i32, premade_moves: &[&str]) -> Vec<(String, u64)> {
    let mut board = Bitboard::new_default();

    for premade_move in premade_moves {
        let parsed_move = Move::from_text(premade_move.trim(), &board);
        board.make_move_active_color(&parsed_move);
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_moves_active_color(&mut moves);

    let mut result = Vec::<(String, u64)>::new();
    for r#move in &moves[0..moves_count] {
        board.make_move_active_color(r#move);

        let moves_count = match board.active_color {
            WHITE => run_internal::<WHITE, BLACK>(depth - 1, &mut board),
            BLACK => run_internal::<BLACK, WHITE>(depth - 1, &mut board),
            _ => panic!("Invalid value: board.active_color={}", board.active_color),
        };

        result.push((r#move.to_text(), moves_count));
        board.undo_move_active_color(r#move);
    }

    result
}

fn run_internal<const COLOR: u8, const ENEMY_COLOR: u8>(depth: i32, board: &mut Bitboard) -> u64 {
    if depth <= 0 {
        return 1;
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_moves::<COLOR>(&mut moves);

    let mut count = 0;
    for r#move in &moves[0..moves_count] {
        board.make_move::<COLOR, ENEMY_COLOR>(r#move);

        if !board.is_king_checked::<COLOR>() {
            count += run_internal::<ENEMY_COLOR, COLOR>(depth - 1, board)
        }

        board.undo_move::<COLOR, ENEMY_COLOR>(r#move);
    }

    count
}
