use super::*;
use crate::state::movescan::Move;

const KILLER_SLOTS: usize = 3;

pub struct KillersTable {
    pub table: [[Move; KILLER_SLOTS]; MAX_DEPTH as usize],
}

impl KillersTable {
    pub fn add(&mut self, ply: u16, r#move: Move) {
        for slot_index in (1..KILLER_SLOTS).rev() {
            self.table[ply as usize][slot_index] = self.table[ply as usize][slot_index - 1];
        }

        self.table[ply as usize][0] = r#move;
    }

    pub fn exists(&self, ply: u16, r#move: Move) -> bool {
        for slot_index in 0..KILLER_SLOTS {
            if self.table[ply as usize][slot_index] == r#move {
                return true;
            }
        }

        false
    }
}

impl Default for KillersTable {
    fn default() -> Self {
        KillersTable {
            table: [[Default::default(); KILLER_SLOTS]; MAX_DEPTH as usize],
        }
    }
}
