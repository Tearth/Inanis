use crate::state::movescan::Move;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;

pub struct CountermovesTable {
    pub table: [[CountermovesTableEntry; 64]; 64],
}

pub struct CountermovesTableEntry {
    pub data: AtomicU16,
}

pub struct CountermovesTableResult {
    pub r#move: Move,
}

impl CountermovesTable {
    pub fn add(&self, previous_move: Move, r#move: Move) {
        self.table[previous_move.get_from()][previous_move.get_to()].set_data(r#move);
    }

    pub fn get(&self, previous_move: Move) -> Move {
        let entry = &self.table[previous_move.get_from()][previous_move.get_to()];
        let entry_data = entry.get_data();

        entry_data.r#move
    }
}

impl Default for CountermovesTable {
    fn default() -> Self {
        const INIT_1: CountermovesTableEntry = CountermovesTableEntry::new_const();
        const INIT_2: [CountermovesTableEntry; 64] = [INIT_1; 64];

        CountermovesTable { table: [INIT_2; 64] }
    }
}

impl CountermovesTableEntry {
    pub const fn new_const() -> Self {
        Self { data: AtomicU16::new(0) }
    }

    pub fn set_data(&self, r#move: Move) {
        self.data.store(r#move.data, Ordering::Relaxed);
    }

    pub fn get_data(&self) -> CountermovesTableResult {
        CountermovesTableResult::new(self.data.load(Ordering::Relaxed))
    }
}

impl Default for CountermovesTableEntry {
    fn default() -> Self {
        Self { data: AtomicU16::new(0) }
    }
}

impl Clone for CountermovesTableEntry {
    fn clone(&self) -> Self {
        Self { data: AtomicU16::new(self.data.load(Ordering::Relaxed)) }
    }
}

impl CountermovesTableResult {
    pub fn new(data: u16) -> Self {
        Self { r#move: Move::new_from_raw(data) }
    }
}
