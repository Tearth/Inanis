use crate::state::movescan::Move;
use std::alloc;
use std::alloc::Layout;
use std::mem;

pub struct CountermovesTable {
    pub table: Box<[[CountermovesTableEntry; 64]; 64]>,
}

pub struct CountermovesTableEntry {
    pub r#move: Move,
}

impl CountermovesTable {
    /// Adds countermove `r#move` as response to `previous_move`.
    pub fn add(&mut self, previous_move: Move, r#move: Move) {
        self.table[previous_move.get_from()][previous_move.get_to()].r#move = r#move;
    }

    /// Gets countermove for `previous_move`.
    pub fn get(&self, previous_move: Move) -> Move {
        self.table[previous_move.get_from()][previous_move.get_to()].r#move
    }
}

impl Default for CountermovesTable {
    /// Constructs a default instance of [CountermovesTable] with zeroed elements.
    fn default() -> Self {
        unsafe {
            let size = mem::size_of::<CountermovesTableEntry>();
            let ptr = alloc::alloc_zeroed(Layout::from_size_align(64 * 64 * size, size).unwrap());
            Self { table: Box::from_raw(ptr as *mut [[CountermovesTableEntry; 64]; 64]) }
        }
    }
}
