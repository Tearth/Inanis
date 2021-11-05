use crate::state::movescan::Move;

const KILLER_SLOTS: usize = 3;

pub struct KillersTable {
    pub table: [[[Move; KILLER_SLOTS]; 32]; 2],
}

impl KillersTable {
    pub fn add(&mut self, color: u8, ply: u16, r#move: Move) {
        for slot_index in (1..KILLER_SLOTS).rev() {
            self.table[color as usize][ply as usize][slot_index] = self.table[color as usize][ply as usize][slot_index - 1];
        }

        self.table[color as usize][ply as usize][0] = r#move;
    }

    pub fn exists(&self, color: u8, ply: u16, r#move: Move) -> bool {
        for slot_index in 0..KILLER_SLOTS {
            if self.table[color as usize][ply as usize][slot_index] == r#move {
                return true;
            }
        }

        false
    }
}

impl Default for KillersTable {
    fn default() -> Self {
        KillersTable {
            table: [[[Default::default(); KILLER_SLOTS]; 32]; 2],
        }
    }
}
