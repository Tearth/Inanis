use crate::board::movescan::Move;
use crate::board::representation::Bitboard;
use crate::cache::perft::PerftHashTable;
use crate::run_internal;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::u64;

use super::common::PerftContext;

pub fn run(depth: i32, board: &mut Bitboard) -> Vec<(String, u64)> {
    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_moves_active_color(&mut moves);

    let hashtable = Arc::new(PerftHashTable::new(0));
    let mut context = PerftContext::new(board, &hashtable, false, false);
    let mut result = Vec::<(String, u64)>::new();

    for r#move in &moves[0..moves_count] {
        context.board.make_move_active_color(r#move);

        result.push((
            r#move.to_text(),
            run_internal!(context.board.active_color, &mut context, depth - 1, false),
        ));

        context.board.undo_move_active_color(r#move);
    }

    result
}
