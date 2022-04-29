use std::sync::atomic::{AtomicU16, Ordering};

use super::*;
use crate::state::movescan::Move;

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
    /// Adds a new killer `r#move` at the level specified by `ply` value. Maximum amount of slots for each of them is specified by
    /// [KILLER_SLOTS] constant, and newest entries have always a priority over old ones.
    pub fn add(&self, ply: u16, r#move: Move) {
        for slot_index in (1..KILLER_SLOTS).rev() {
            let entry = &self.table[ply as usize][slot_index - 1];
            let entry_data = entry.get_data();

            self.table[ply as usize][slot_index].set_data(entry_data.r#move);
        }

        self.table[ply as usize][0].set_data(r#move);
    }

    /// Checks if killer `r#move` exists at the level specified by `ply`.
    pub fn exists(&self, ply: u16, r#move: Move) -> bool {
        for slot_index in 0..KILLER_SLOTS {
            let entry = &self.table[ply as usize][slot_index];
            let entry_data = entry.get_data();

            if entry_data.r#move == r#move {
                return true;
            }
        }

        false
    }

    /// Ages killer table by shifting all ply levels by two positions up, to ensure that killer moves inside match board after two halfmoves.
    pub fn age_moves(&self) {
        for ply in 3..MAX_DEPTH {
            for slot_index in 0..KILLER_SLOTS {
                let entry = &self.table[ply as usize][slot_index];
                let entry_data = entry.get_data();

                self.table[(ply as usize) - 2][slot_index].set_data(entry_data.r#move);
            }
        }

        for ply in MAX_DEPTH - 2..MAX_DEPTH {
            for slot_index in 0..KILLER_SLOTS {
                self.table[ply as usize][slot_index].set_data(Default::default());
            }
        }
    }
}

impl Default for KillersTable {
    /// Constructs a default instance of [KillersTable] with zeroed elements.
    fn default() -> Self {
        const INIT_1: KillersTableEntry = KillersTableEntry::new_const();
        const INIT_2: [KillersTableEntry; KILLER_SLOTS] = [INIT_1; KILLER_SLOTS];

        KillersTable {
            table: [INIT_2; MAX_DEPTH as usize],
        }
    }
}

impl KillersTableEntry {
    /// Constructs a new instance of [KillersTableEntry] with stored `key` and `score`.
    pub fn new(r#move: Move) -> Self {
        Self {
            data: AtomicU16::new(r#move.data),
        }
    }

    pub const fn new_const() -> Self {
        Self { data: AtomicU16::new(0) }
    }

    /// Converts `r#move` into an atomic word, and stores it.
    pub fn set_data(&self, r#move: Move) {
        self.data.store(r#move.data, Ordering::Relaxed);
    }

    /// Loads and parses atomic value into a [KillersTableEntry] struct.
    pub fn get_data(&self) -> KillersTableResult {
        let data = self.data.load(Ordering::Relaxed);
        KillersTableResult {
            r#move: Move::new_from_raw(data),
        }
    }
}

impl Default for KillersTableEntry {
    fn default() -> Self {
        Self { data: AtomicU16::new(0) }
    }
}

impl Clone for KillersTableEntry {
    fn clone(&self) -> Self {
        Self {
            data: AtomicU16::new(self.data.load(Ordering::Relaxed)),
        }
    }
}
