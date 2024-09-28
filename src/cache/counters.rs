use crate::state::movescan::Move;
use crate::utils::assert_fast;
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
        assert_fast!(previous_move.is_some());
        assert_fast!(previous_move.get_from() < 64);
        assert_fast!(previous_move.get_to() < 64);
        assert_fast!(r#move.is_some());

        self.table[previous_move.get_from()][previous_move.get_to()].r#move = r#move;
    }

    /// Gets countermove for `previous_move`.
    pub fn get(&self, previous_move: Move) -> Move {
        assert_fast!(previous_move.get_from() < 64);
        assert_fast!(previous_move.get_to() < 64);

        self.table[previous_move.get_from()][previous_move.get_to()].r#move
    }
}

impl Default for CountermovesTable {
    /// Constructs a default instance of [CountermovesTable] by allocating `64 * 64 * mem::size_of::<CountermovesTableEntry>()`
    /// boxed array with zeroed elements.
    fn default() -> Self {
        const SIZE: usize = mem::size_of::<CountermovesTableEntry>();
        unsafe {
            let ptr = alloc::alloc_zeroed(Layout::from_size_align(64 * 64 * SIZE, SIZE).unwrap());
            Self { table: Box::from_raw(ptr as *mut [[CountermovesTableEntry; 64]; 64]) }
        }
    }
}
