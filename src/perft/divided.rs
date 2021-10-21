use super::context::PerftContext;
use crate::cache::perft::PerftHashTable;
use crate::run_perft;
use crate::state::board::Bitboard;
use crate::state::movescan::Move;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::u64;

pub fn run(depth: i32, board: &mut Bitboard) -> Vec<(String, u64)> {
    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_moves(&mut moves);

    let hashtable = Arc::new(PerftHashTable::new(0));
    let mut context = PerftContext::new(board, &hashtable, false, false);
    let mut result = Vec::<(String, u64)>::new();

    for r#move in &moves[0..moves_count] {
        context.board.make_move(r#move);

        result.push((
            r#move.to_text(),
            run_perft!(context.board.active_color, &mut context, depth - 1, false),
        ));

        context.board.undo_move(r#move);
    }

    result
}
