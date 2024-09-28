use super::context::PerftContext;
use super::*;
use crate::cache::perft::PerftHashTable;
use crate::state::representation::Board;
use std::mem::MaybeUninit;
use std::sync::Arc;

/// Entry point of the fixed-`depth` divided perft, which performs a separate perfts for every possible move in the position specified by `board`.
/// Returns a map with the long notation moves as the key, and calculated nodes count as the associated value.
pub fn run(depth: i32, board: &mut Board) -> Vec<(String, u64)> {
    let mut moves = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
    let moves_count = board.get_all_moves(&mut moves, u64::MAX);

    let hashtable = Arc::new(PerftHashTable::new(0));
    let mut context = PerftContext::new(board, &hashtable, false, false);
    let mut result = Vec::<(String, u64)>::new();

    for r#move in &moves[0..moves_count] {
        let r#move = unsafe { r#move.assume_init() };

        context.board.make_move(r#move);
        result.push((r#move.to_long_notation(), run_internal(&mut context, depth - 1)));
        context.board.undo_move(r#move);
    }

    result
}
