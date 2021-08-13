use crate::board::movescan::Move;
use std::cell::UnsafeCell;
use std::mem;
use std::u64;

bitflags! {
    pub struct TranspositionTableScoreType: u8 {
        const INVALID = 0;
        const EXACT_SCORE = 1;
        const ALPHA_SCORE = 2;
        const BETA_SCORE = 4;
    }
}

pub struct TranspositionTable {
    table: Vec<TranspositionTableEntry>,
    slots: usize,
}

#[derive(Clone, Copy)]
pub struct TranspositionTableEntry {
    pub key: u32,
    pub score: i16,
    pub best_move: Move,
    pub depth: i8,
    pub score_type: TranspositionTableScoreType,
}

impl TranspositionTable {
    pub fn new(size: usize) -> TranspositionTable {
        let buckets = size / mem::size_of::<TranspositionTableEntry>();
        let mut hashtable = TranspositionTable {
            table: Vec::with_capacity(buckets),
            slots: buckets,
        };

        if size != 0 {
            hashtable.table.resize(
                hashtable.slots,
                TranspositionTableEntry::new(0, 0, Move::new_empty(), 0, TranspositionTableScoreType::INVALID),
            );
        }

        hashtable
    }

    pub fn add(&mut self, hash: u64, score: i16, best_move: Move, depth: i8, score_type: TranspositionTableScoreType) {
        self.table[(hash as usize) % self.slots] = TranspositionTableEntry::new((hash >> 32) as u32, score, best_move, depth, score_type);
    }

    pub fn get(&self, hash: u64, depth: i8) -> TranspositionTableEntry {
        self.table[(hash as usize) % self.slots]
    }

    /*pub fn get_usage(&self) -> f32 {
        const RESOLUTION: usize = 10000;
        const BUCKETS_COUNT_TO_CHECK: usize = RESOLUTION / BUCKET_SLOTS;

        let mut filled_entries = 0;

        for bucket_index in 0..BUCKETS_COUNT_TO_CHECK {
            for entry in unsafe { (*self.table.get())[bucket_index].entries } {
                if entry.key != 0 {
                    filled_entries += 1;
                }
            }
        }

        ((filled_entries as f32) / (RESOLUTION as f32)) * 100.0
    }*/
}

impl TranspositionTableEntry {
    pub fn new(key: u32, score: i16, best_move: Move, depth: i8, score_type: TranspositionTableScoreType) -> TranspositionTableEntry {
        TranspositionTableEntry {
            key,
            score,
            best_move,
            depth,
            score_type,
        }
    }
}
