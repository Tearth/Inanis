use crate::engine::*;
use crate::state::movescan::Move;
use std::mem;
use std::u64;

const BUCKET_SLOTS: usize = 5;

bitflags! {
    pub struct TranspositionTableScoreType: u8 {
        const INVALID = 0;
        const EXACT_SCORE = 1;
        const ALPHA_SCORE = 2;
        const BETA_SCORE = 4;
    }
}

pub struct TranspositionTable {
    table: Vec<TranspositionTableBucket>,
    slots: usize,
}

#[repr(align(64))]
#[derive(Clone, Copy)]
struct TranspositionTableBucket {
    pub entries: [TranspositionTableEntry; BUCKET_SLOTS],
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
        let bucket_size = mem::size_of::<TranspositionTableBucket>();
        let buckets = size / bucket_size;
        let mut hashtable = TranspositionTable {
            table: Vec::with_capacity(buckets),
            slots: buckets,
        };

        if size != 0 {
            hashtable.table.resize(hashtable.slots, Default::default());
        }

        hashtable
    }

    pub fn add(&mut self, hash: u64, mut score: i16, best_move: Move, depth: i8, ply: u16, score_type: TranspositionTableScoreType) {
        let key = self.get_key(hash);
        let mut bucket = self.table[(hash as usize) % self.slots];
        let mut smallest_depth = bucket.entries[0].depth as u8;
        let mut smallest_depth_index = 0;

        for entry_index in 0..BUCKET_SLOTS {
            if bucket.entries[entry_index].key == key {
                smallest_depth_index = entry_index;
                break;
            }

            let entry_depth = bucket.entries[entry_index].depth as u8;
            if entry_depth < smallest_depth {
                smallest_depth = entry_depth;
                smallest_depth_index = entry_index;
            }
        }

        if is_score_near_checkmate(score) {
            if score > 0 {
                score += ply as i16;
            } else {
                score -= ply as i16;
            }
        }

        bucket.entries[smallest_depth_index] = TranspositionTableEntry::new(key, score, best_move, depth, score_type);
        self.table[(hash as usize) % self.slots] = bucket;
    }

    pub fn get(&self, hash: u64, ply: u16, collision: &mut bool) -> Option<TranspositionTableEntry> {
        let key = self.get_key(hash);
        let bucket = self.table[(hash as usize) % self.slots];
        let mut entry_with_key_present = false;

        for entry_index in 0..BUCKET_SLOTS {
            let mut entry = bucket.entries[entry_index];
            if entry.key == key {
                if is_score_near_checkmate(entry.score) {
                    if entry.score > 0 {
                        entry.score -= ply as i16;
                    } else {
                        entry.score += ply as i16;
                    }
                }

                return Some(entry);
            } else if entry.key != 0 {
                entry_with_key_present = true;
            }
        }

        if entry_with_key_present {
            *collision = true;
        }

        None
    }

    pub fn get_best_move(&self, hash: u64) -> Option<Move> {
        let mut collision = false;
        self.get(hash, 0, &mut collision).map(|entry| entry.best_move)
    }

    pub fn get_usage(&self) -> f32 {
        const RESOLUTION: usize = 10000;
        const BUCKETS_COUNT_TO_CHECK: usize = RESOLUTION / BUCKET_SLOTS;
        let mut filled_entries = 0;

        for bucket_index in 0..BUCKETS_COUNT_TO_CHECK {
            for entry_index in 0..BUCKET_SLOTS {
                let entry = self.table[bucket_index].entries[entry_index];
                if entry.key != 0 {
                    filled_entries += 1;
                }
            }
        }

        ((filled_entries as f32) / (RESOLUTION as f32)) * 100.0
    }

    pub fn clear(&mut self) {
        self.table.clear();
        self.table.resize(self.slots, Default::default());
    }

    fn get_key(&self, hash: u64) -> u32 {
        (hash >> 32) as u32
    }
}

impl Default for TranspositionTableBucket {
    fn default() -> Self {
        TranspositionTableBucket {
            entries: [Default::default(); BUCKET_SLOTS],
        }
    }
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

impl Default for TranspositionTableEntry {
    fn default() -> Self {
        TranspositionTableEntry::new(0, 0, Default::default(), 0, TranspositionTableScoreType::INVALID)
    }
}
