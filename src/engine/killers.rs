use super::*;
use crate::state::movescan::Move;

const KILLER_SLOTS: usize = 3;

pub struct KillersTable {
    pub table: [[Move; KILLER_SLOTS]; MAX_DEPTH as usize],
}

impl KillersTable {
    /// Adds a new killer `r#move` at the level specified by `ply` value. Maximum amount of slots for each of them is specified by
    /// [KILLER_SLOTS] constant, and newest entries have always a priority over old ones.
    pub fn add(&mut self, ply: u16, r#move: Move) {
        for slot_index in (1..KILLER_SLOTS).rev() {
            self.table[ply as usize][slot_index] = self.table[ply as usize][slot_index - 1];
        }

        self.table[ply as usize][0] = r#move;
    }

    /// Checks if killer `r#move` exists at the level specified by `ply`.
    pub fn exists(&self, ply: u16, r#move: Move) -> bool {
        for slot_index in 0..KILLER_SLOTS {
            if self.table[ply as usize][slot_index] == r#move {
                return true;
            }
        }

        false
    }

    /// Ages killer table by shifting all ply levels by two positions up, to ensure that killer moves inside match board after two halfmoves.
    pub fn age_moves(&mut self) {
        for ply in 3..MAX_DEPTH {
            self.table[(ply as usize) - 2] = self.table[ply as usize];
        }

        self.table[(MAX_DEPTH - 2) as usize] = [Default::default(); KILLER_SLOTS];
        self.table[(MAX_DEPTH - 1) as usize] = [Default::default(); KILLER_SLOTS];
    }
}

impl Default for KillersTable {
    /// Constructs a default instance of [KillersTable] with zeroed elements.
    fn default() -> Self {
        KillersTable {
            table: [[Default::default(); KILLER_SLOTS]; MAX_DEPTH as usize],
        }
    }
}
