use crate::engine::*;
use crate::state::movescan::Move;

const KILLER_SLOTS: usize = 2;

pub struct KillersTable {
    pub table: [[KillersTableEntry; KILLER_SLOTS]; MAX_DEPTH as usize],
}

pub struct KillersTableEntry {
    pub data: Move,
}

impl KillersTable {
    /// Adds a new killer `r#move` at the level specified by `ply` value. Maximal amount of slots for each of them is set by
    /// [KILLER_SLOTS] constant, and newer entries have always a priority over old ones. If there's already exactly the same
    /// move in the slot 0, the table is not changed.
    pub fn add(&mut self, ply: u16, r#move: Move) {
        if ply >= MAX_DEPTH as u16 || self.table[ply as usize][0].data == r#move {
            return;
        }

        for slot_index in (1..KILLER_SLOTS).rev() {
            let entry = &mut self.table[ply as usize][slot_index - 1];
            self.table[ply as usize][slot_index].data = entry.data;
        }

        self.table[ply as usize][0].data = r#move;
    }

    /// Gets all killer moves at the level specified by `ply`.
    pub fn get(&self, ply: u16) -> [Move; KILLER_SLOTS] {
        let mut result = [Default::default(); KILLER_SLOTS];

        if ply >= MAX_DEPTH as u16 {
            return result;
        }

        for (index, slot) in self.table[ply as usize].iter().enumerate() {
            result[index] = slot.data
        }

        result
    }

    /// Ages killer table by shifting all ply levels by two positions up, to ensure that killer moves inside match board after two halfmoves.
    pub fn age_moves(&mut self) {
        for row in 2..MAX_DEPTH {
            for slot_index in 0..KILLER_SLOTS {
                let entry = &self.table[row as usize][slot_index];
                self.table[(row as usize) - 2][slot_index].data = entry.data;
            }
        }

        for ply in MAX_DEPTH - 2..MAX_DEPTH {
            for entry in &mut self.table[ply as usize] {
                entry.data = Default::default();
            }
        }
    }
}

impl Default for KillersTable {
    /// Constructs a default instance of [KillersTable] with zeroed elements.
    fn default() -> Self {
        const INIT_1: KillersTableEntry = KillersTableEntry::new_const();
        const INIT_2: [KillersTableEntry; KILLER_SLOTS] = [INIT_1; KILLER_SLOTS];

        Self { table: [INIT_2; MAX_DEPTH as usize] }
    }
}

impl KillersTableEntry {
    /// Constructs a new instance of [KillersTableEntry] with zeroed values.
    pub const fn new_const() -> Self {
        Self { data: Move::new_from_raw(0) }
    }
}

impl Default for KillersTableEntry {
    /// Constructs a default instance of [KillersTableEntry] with zeroed elements.
    fn default() -> Self {
        Self { data: Move::new_from_raw(0) }
    }
}
