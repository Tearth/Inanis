use crate::state::movescan::Move;

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
        let entry = &self.table[previous_move.get_from()][previous_move.get_to()];
        entry.r#move
    }
}

impl Default for CountermovesTable {
    /// Constructs a default instance of [CountermovesTable] with zeroed elements.
    fn default() -> Self {
        const INIT_1: CountermovesTableEntry = CountermovesTableEntry::new_const();
        const INIT_2: [CountermovesTableEntry; 64] = [INIT_1; 64];

        CountermovesTable { table: Box::new([INIT_2; 64]) }
    }
}

impl CountermovesTableEntry {
    /// Constructs a new instance of [CountermovesTableEntry] with zeroed values.
    pub const fn new_const() -> Self {
        Self { r#move: Move::new_from_raw(0) }
    }
}

impl Default for CountermovesTableEntry {
    /// Constructs a default instance of [CountermovesTableEntry] with zeroed elements.
    fn default() -> Self {
        Self { r#move: Default::default() }
    }
}
