use crate::engine::*;
use crate::state::movescan::Move;
use crate::utils::assert_fast;
use std::alloc;
use std::alloc::Layout;
use std::mem;

const KILLER_SLOTS: usize = 2;

pub struct KTable {
    pub table: Box<[[KTableEntry; KILLER_SLOTS]; MAX_DEPTH as usize]>,
}

pub struct KTableEntry {
    pub data: Move,
}

impl KTable {
    /// Adds a new killer `r#move` at the level specified by `ply` value. Maximal amount of slots for each of them is set by
    /// [KILLER_SLOTS] constant, and newer entries have always a priority over old ones. If there's already exactly the same
    /// move in the slot 0, the table is not changed.
    pub fn add(&mut self, ply: u16, r#move: Move) {
        assert_fast!(r#move.is_some());

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
        let mut result = [Move::default(); KILLER_SLOTS];

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
                entry.data = Move::default();
            }
        }
    }
}

impl Default for KTable {
    /// Constructs a default instance of [KTable] by allocating `KILLER_SLOTS * MAX_DEPTH * mem::size_of::<KTableEntry>()`
    /// boxed array with zeroed elements.
    fn default() -> Self {
        const SIZE: usize = mem::size_of::<KTableEntry>();
        unsafe {
            let ptr = alloc::alloc_zeroed(Layout::from_size_align(KILLER_SLOTS * MAX_DEPTH as usize * SIZE, SIZE).unwrap());
            Self { table: Box::from_raw(ptr as *mut [[KTableEntry; KILLER_SLOTS]; MAX_DEPTH as usize]) }
        }
    }
}
