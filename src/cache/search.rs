use crate::engine;
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
    /// Constructs a new instance of [TranspositionTable] by allocating `size` bytes of memory.
    pub fn new(size: usize) -> TranspositionTable {
        let bucket_size = mem::size_of::<TranspositionTableBucket>();
        let mut hashtable = TranspositionTable {
            table: Vec::with_capacity(size / bucket_size),
        };

        if size != 0 {
            hashtable.table.resize(hashtable.table.capacity(), Default::default());
        }

        hashtable
    }

    /// Adds a new entry (storing the key, `score`, `best_move`, `depth`, `ply` and `score_type`) using `hash % self.table.len()` formula to calculate an index of the bucket.
    /// Replacement strategy considers a few elements to optimize memory usage and prioritizes slots to replace as follows:
    ///  - slots with the same key as the new entry
    ///  - empty slots
    ///  - slots with the smallest depth (to ensure that the table is not clogged with entries that will be rarely read)
    ///  - slots with the biggest age counter (to ensure that old and possibly outdated entries are not preventing us from adding a new one)
    ///
    /// This function takes care of converting mate `score` using passed `ply`.
    pub fn add(&mut self, hash: u64, mut score: i16, best_move: Move, depth: i8, ply: u16, score_type: TranspositionTableScoreType) {
        let key = self.get_key(hash);
        let index = (hash as usize) % self.table.len();
        let mut bucket = self.table[index];
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

        if engine::is_score_near_checkmate(score) {
            if score > 0 {
                score += ply as i16;
            } else {
                score -= ply as i16;
            }
        }

        bucket.entries[target_index] = TranspositionTableEntry::new(key, score, best_move, depth, score_type);
        self.table[index] = bucket;
    }

    /// Gets a wanted entry using `hash % self.table.len()` formula to calculate an index of the bucket. This function takes care of converting
    /// mate `score` using passed `ply`. Returns [None] if `hash` is incompatible with the stored key (and sets `collision` flag to true).
    pub fn get(&self, hash: u64, ply: u16, collision: &mut bool) -> Option<TranspositionTableEntry> {
        let key = self.get_key(hash);
        let index = (hash as usize) % self.table.len();
        let bucket = self.table[index];
        let mut entry_with_key_present = false;

        for entry_index in 0..BUCKET_SLOTS {
            let mut entry = bucket.entries[entry_index];
            if entry.key == key {
                if engine::is_score_near_checkmate(entry.score) {
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

    /// Gets an entry's best move using `hash % self.table.len()` formula to calculate an index of the bucket.
    /// Returns [None] if `hash` is incompatible with the stored key.
    pub fn get_best_move(&self, hash: u64) -> Option<Move> {
        let mut collision = false;
        self.get(hash, 0, &mut collision).map(|entry| entry.best_move)
    }

    /// Calculates an approximate percentage usage of the table, based on the first 10000 entries.
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

    /// Increments ann age of all entries stored in the table. If age's value is equal to 31, it gets purged to make more space for a new entries.
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

    /// Calculates a key for the `hash` by taking the last 16 bits of it.
    fn get_key(&self, hash: u64) -> u16 {
        (hash >> 48) as u16
    }
}

impl Default for TranspositionTableBucket {
    /// Constructs a default instance of [TranspositionTableBucket] with zeroed elements.
    fn default() -> Self {
        TranspositionTableBucket {
            entries: [Default::default(); BUCKET_SLOTS],
        }
    }
}

impl TranspositionTableEntry {
    /// Constructs a new instance of [TranspositionTableEntry] with stored `key`, `score`, `best_move`, `depth` and `r#type`.
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

    /// Gets an entry flag by reading `self.type_age` field and parsing it into [TranspositionTableScoreType] type.
    pub fn get_flags(&self) -> TranspositionTableScoreType {
        TranspositionTableScoreType::from_bits(self.type_age & 7).unwrap()
    }

    /// Gets an entry age by reading `self.type_age` field.
    pub fn get_age(&self) -> u8 {
        self.type_age >> 3
    }

    // Sets an entry age to the `age` value.
    pub fn set_age(&mut self, age: u8) {
        self.type_age = (self.type_age & 7) | (age << 3);
    }
}

impl Default for TranspositionTableEntry {
    /// Constructs a default instance of [TranspositionTableEntry] with zeroed elements.
    fn default() -> Self {
        TranspositionTableEntry::new(0, 0, Default::default(), 0, TranspositionTableScoreType::INVALID)
    }
}
