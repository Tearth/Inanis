use crate::cache::perft::PerftHashTable;
use crate::state::board::Bitboard;
use std::sync::Arc;

pub struct PerftContext<'a> {
    pub board: &'a mut Bitboard,
    pub hashtable: &'a Arc<PerftHashTable>,
    pub check_integrity: bool,
    pub fast: bool,
}

impl<'a> PerftContext<'a> {
    pub fn new(board: &'a mut Bitboard, hashtable: &'a Arc<PerftHashTable>, check_integrity: bool, fast: bool) -> PerftContext<'a> {
        PerftContext {
            board,
            hashtable,
            check_integrity,
            fast,
        }
    }
}