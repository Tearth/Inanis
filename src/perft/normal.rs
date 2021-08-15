use crate::cache::perft::PerftHashTable;
use crate::perft::context::PerftContext;
use crate::run_perft;
use crate::state::board::Bitboard;
use std::sync::Arc;
use std::u64;

pub fn run(depth: i32, board: &mut Bitboard, check_integrity: bool) -> u64 {
    let hashtable = Arc::new(PerftHashTable::new(0));
    let mut context = PerftContext::new(board, &hashtable, check_integrity, false);

    run_perft!(context.board.active_color, &mut context, depth, false)
}
