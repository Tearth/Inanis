use super::context::PerftStats;
use super::*;
use crate::cache::perft::PerftHashTable;
use crate::perft::context::PerftContext;
use crate::state::representation::Board;
use std::sync::Arc;

pub struct NormalPerftResult {
    pub nodes: u64,
    pub stats: PerftStats,
}

impl NormalPerftResult {
    /// Constructs a new instance of [NormalPerftResult] with stored `nodes` and `stats`.
    pub fn new(nodes: u64, stats: PerftStats) -> Self {
        Self { nodes, stats }
    }
}

/// Entry point of the fixed-`depth` simple perft. Use `check_integrity` to allow panics when internal state becomes invalid due to some bug.
pub fn run(depth: i32, board: &mut Board, check_integrity: bool) -> NormalPerftResult {
    let hashtable = Arc::new(PerftHashTable::new(0));
    let mut context = PerftContext::new(board, &hashtable, check_integrity, false);

    NormalPerftResult::new(run_internal(&mut context, depth), context.stats)
}
