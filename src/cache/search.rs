use crate::engine::*;
use crate::state::movescan::Move;
use std::mem;
use std::u64;

const BUCKET_SLOTS: usize = 8;

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
    pub key: u16,
    pub score: i16,
    pub best_move: Move,
    pub depth: i8,
    pub type_age: u8,
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
        let mut smallest_depth = u8::MAX;
        let mut smallest_depth_index = usize::MAX;
        let mut oldest_entry_age = 0;
        let mut oldest_entry_index = usize::MAX;

        for entry_index in 0..BUCKET_SLOTS {
            if bucket.entries[entry_index].key == key {
                smallest_depth_index = entry_index;
                oldest_entry_index = entry_index;
                break;
            }

            if bucket.entries[entry_index].depth == 0 {
                smallest_depth = 0;
                smallest_depth_index = entry_index;
                oldest_entry_age = u8::MAX;
                oldest_entry_index = entry_index;
                continue;
            }

            let entry_age = bucket.entries[entry_index].get_age();
            if entry_age > oldest_entry_age {
                oldest_entry_age = entry_age;
                oldest_entry_index = entry_index;
                continue;
            }

            let entry_depth = bucket.entries[entry_index].depth as u8;
            if entry_depth < smallest_depth {
                smallest_depth = entry_depth;
                smallest_depth_index = entry_index;
                continue;
            }
        }

        let target_index = if oldest_entry_index != usize::MAX {
            oldest_entry_index
        } else {
            smallest_depth_index
        };

        if is_score_near_checkmate(score) {
            if score > 0 {
                score += ply as i16;
            } else {
                score -= ply as i16;
            }
        }

        bucket.entries[target_index] = TranspositionTableEntry::new(key, score, best_move, depth, score_type);
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

    pub fn age_entries(&mut self) {
        for bucket_index in 0..self.table.len() {
            for entry_index in 0..BUCKET_SLOTS {
                let mut entry = self.table[bucket_index].entries[entry_index];
                if entry.depth > 0 {
                    if entry.get_age() == 31 {
                        self.table[bucket_index].entries[entry_index] = Default::default();
                    } else {
                        entry.set_age(entry.get_age() + 1);
                        self.table[bucket_index].entries[entry_index] = entry;
                    }
                }
            }
        }
    }

    fn get_key(&self, hash: u64) -> u16 {
        (hash >> 48) as u16
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
    pub fn new(key: u16, score: i16, best_move: Move, depth: i8, r#type: TranspositionTableScoreType) -> TranspositionTableEntry {
        let type_age = r#type.bits;
        TranspositionTableEntry {
            key,
            score,
            best_move,
            depth,
            type_age,
        }
    }

    pub fn get_flags(&self) -> TranspositionTableScoreType {
        TranspositionTableScoreType::from_bits(self.type_age & 7).unwrap()
    }

    pub fn get_age(&self) -> u8 {
        self.type_age >> 3
    }

    pub fn set_age(&mut self, age: u8) {
        self.type_age = (self.type_age & 7) | (age << 3);
    }
}

impl Default for TranspositionTableEntry {
    fn default() -> Self {
        TranspositionTableEntry::new(0, 0, Default::default(), 0, TranspositionTableScoreType::INVALID)
    }
}
