use crate::engine::*;
use crate::state::movescan::Move;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;

const KILLER_SLOTS: usize = 3;

#[derive(Clone)]
pub struct KillersTable {
    pub table: [[KillersTableEntry; KILLER_SLOTS]; MAX_DEPTH as usize],
}

pub struct KillersTableEntry {
    pub data: AtomicU16,
}

pub struct KillersTableResult {
    pub r#move: Move,
}

impl KillersTable {
    /// Adds a new killer `r#move` at the level specified by `ply` value. Maximum amount of slots for each of them is set by
    /// [KILLER_SLOTS] constant, and newer entries have always a priority over old ones.
    pub fn add(&self, ply: u16, r#move: Move) {
        for slot_index in (1..KILLER_SLOTS).rev() {
            let entry = &self.table[ply as usize][slot_index - 1];
            let entry_data = entry.get_data();

            self.table[ply as usize][slot_index].set_data(entry_data.r#move);
        }

        self.table[ply as usize][0].set_data(r#move);
    }

    /// Gets all killer moves at the level specified by `ply`.
    pub fn get(&self, ply: u16) -> [Move; KILLER_SLOTS] {
        let mut result = [Default::default(); KILLER_SLOTS];
        for (index, slot) in self.table[ply as usize].iter().enumerate() {
            result[index] = slot.get_data().r#move;
        }

        result
    }

    /// Ages killer table by shifting all ply levels by two positions up, to ensure that killer moves inside match board after two halfmoves.
    pub fn age_moves(&self) {
        for row in 2..MAX_DEPTH {
            for slot_index in 0..KILLER_SLOTS {
                let entry = &self.table[row as usize][slot_index];
                let entry_data = entry.get_data();

                self.table[(row as usize) - 2][slot_index].set_data(entry_data.r#move);
            }
        }

        for ply in MAX_DEPTH - 2..MAX_DEPTH {
            for entry in &self.table[ply as usize] {
                entry.set_data(Default::default());
            }
        }
    }
}

impl Default for KillersTable {
    /// Constructs a default instance of [KillersTable] with zeroed elements.
    fn default() -> Self {
        const INIT_1: KillersTableEntry = KillersTableEntry::new_const();
        const INIT_2: [KillersTableEntry; KILLER_SLOTS] = [INIT_1; KILLER_SLOTS];

        Self {
            table: [INIT_2; MAX_DEPTH as usize],
        }
    }
}

impl KillersTableEntry {
    /// Constructs a new instance of [KillersTableEntry] with zeroed values.
    pub const fn new_const() -> Self {
        Self { data: AtomicU16::new(0) }
    }

    /// Converts `r#move` into an atomic word, and stores it.
    pub fn set_data(&self, r#move: Move) {
        self.data.store(r#move.data, Ordering::Relaxed);
    }

    /// Loads and parses atomic value into a [KillersTableEntry] struct.
    pub fn get_data(&self) -> KillersTableResult {
        KillersTableResult {
            r#move: Move::new_from_raw(self.data.load(Ordering::Relaxed)),
        }
    }
}

impl Default for KillersTableEntry {
    /// Constructs a default instance of [KillersTableEntry] with zeroed elements.
    fn default() -> Self {
        Self { data: AtomicU16::new(0) }
    }
}

impl Clone for KillersTableEntry {
    /// Clones [KillersTableEntry] by creating a new atomic (with the original value).
    fn clone(&self) -> Self {
        Self {
            data: AtomicU16::new(self.data.load(Ordering::Relaxed)),
        }
    }
}
