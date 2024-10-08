use crate::cache::perft::PerftHashTable;
use crate::state::representation::Board;
use std::sync::Arc;

pub struct PerftContext<'a> {
    pub board: &'a mut Board,
    pub hashtable: &'a Arc<PerftHashTable>,
    pub check_integrity: bool,
    pub fast: bool,
    pub stats: PerftStats,
}

#[derive(Default)]
pub struct PerftStats {
    pub captures: u64,
    pub en_passants: u64,
    pub castles: u64,
    pub promotions: u64,
    pub checks: u64,
}

impl<'a> PerftContext<'a> {
    /// Constructs a new instance of [PerftContext] with `board` as initial state and `hashtable`. Use `check_integrity` to allow panics when internal state
    /// becomes invalid due to some bug, and `fast` to allow `hashtable` work.
    pub fn new(board: &'a mut Board, hashtable: &'a Arc<PerftHashTable>, check_integrity: bool, fast: bool) -> Self {
        Self { board, hashtable, check_integrity, fast, stats: Default::default() }
    }
}
