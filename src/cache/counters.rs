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
    /// Adds countermove `r#move` as response to `previous_move`.
    pub fn add(&self, previous_move: Move, r#move: Move) {
        self.table[previous_move.get_from()][previous_move.get_to()].set_data(r#move);
    }

    /// Gets countermove for `previous_move`.
    pub fn get(&self, previous_move: Move) -> Move {
        let entry = &self.table[previous_move.get_from()][previous_move.get_to()];
        let entry_data = entry.get_data();

        entry_data.r#move
    }
}

impl Default for CountermovesTable {
    /// Constructs a default instance of [CountermovesTable] with zeroed elements.
    fn default() -> Self {
        const INIT_1: CountermovesTableEntry = CountermovesTableEntry::new_const();
        const INIT_2: [CountermovesTableEntry; 64] = [INIT_1; 64];

        CountermovesTable { table: [INIT_2; 64] }
    }
}

impl CountermovesTableEntry {
    /// Constructs a new instance of [CountermovesTableEntry] with zeroed values.
    pub const fn new_const() -> Self {
        Self { data: AtomicU16::new(0) }
    }

    /// Converts `r#move` into an atomic word, and stores it.
    pub fn set_data(&self, r#move: Move) {
        self.data.store(r#move.data, Ordering::Relaxed);
    }

    /// Loads and parses atomic value into a [CountermovesTableResult] struct.
    pub fn get_data(&self) -> CountermovesTableResult {
        CountermovesTableResult::new(self.data.load(Ordering::Relaxed))
    }
}

impl Default for CountermovesTableEntry {
    /// Constructs a default instance of [CountermovesTableEntry] with zeroed elements.
    fn default() -> Self {
        Self { data: AtomicU16::new(0) }
    }
}

impl CountermovesTableResult {
    /// Constructs a new instance of [CountermovesTableResult] with stored `data`.
    pub fn new(data: u16) -> Self {
        Self { r#move: Move::new_from_raw(data) }
    }
}
