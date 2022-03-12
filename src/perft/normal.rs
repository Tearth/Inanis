use super::*;
use crate::cache::perft::PerftHashTable;
use crate::perft::context::PerftContext;
use crate::state::board::Bitboard;
use std::sync::Arc;
use std::u64;

/// Entry point of the fixed-`depth` simple perft. Use `check_integrity` to allow panics when internal state becomes invalid due to some bug.
pub fn run(depth: i32, board: &mut Bitboard, check_integrity: bool) -> u64 {
    let hashtable = Arc::new(PerftHashTable::new(0));
    let mut context = PerftContext::new(board, &hashtable, check_integrity, false);

    run_internal(&mut context, depth)
}
