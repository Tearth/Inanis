use crate::engine;
use crate::state::movescan::Move;
use std::mem;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
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
#[derive(Clone)]
struct TranspositionTableBucket {
    pub entries: [TranspositionTableEntry; BUCKET_SLOTS],
}

pub struct TranspositionTableEntry {
    pub key_data: AtomicU64,
}

pub struct TranspositionTableResult {
    pub key: u16,
    pub score: i16,
    pub best_move: Move,
    pub depth: i8,
    pub r#type: TranspositionTableScoreType,
    pub age: u8,
}

impl TranspositionTable {
    /// Constructs a new instance of [TranspositionTable] by allocating `size` bytes of memory.
    pub fn new(size: usize) -> Self {
        let bucket_size = mem::size_of::<TranspositionTableBucket>();
        let mut hashtable = Self {
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
    pub fn add(&self, hash: u64, mut score: i16, best_move: Move, depth: i8, ply: u16, score_type: TranspositionTableScoreType) {
        let key = self.get_key(hash);
        let index = (hash as usize) % self.table.len();
        let bucket = &self.table[index];

        let mut smallest_depth = i8::MAX;
        let mut smallest_depth_index = usize::MAX;
        let mut oldest_entry_age = 0;
        let mut oldest_entry_index = usize::MAX;

        for (entry_index, entry) in bucket.entries.iter().enumerate() {
            let entry_data = entry.get_data();

            if entry_data.key == key {
                smallest_depth_index = entry_index;
                oldest_entry_index = entry_index;
                break;
            }

            if entry_data.depth == 0 {
                smallest_depth = 0;
                smallest_depth_index = entry_index;
                oldest_entry_age = u8::MAX;
                oldest_entry_index = entry_index;
                continue;
            }

            if entry_data.age > oldest_entry_age {
                oldest_entry_age = entry_data.age;
                oldest_entry_index = entry_index;
                continue;
            }

            if entry_data.depth < smallest_depth {
                smallest_depth = entry_data.depth;
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

        bucket.entries[target_index].set_data(key, score, best_move, depth, score_type, 0);
    }

    /// Gets a wanted entry using `hash % self.table.len()` formula to calculate an index of the bucket. This function takes care of converting
    /// mate `score` using passed `ply`. Returns [None] if `hash` is incompatible with the stored key (and sets `collision` flag to true).
    pub fn get(&self, hash: u64, ply: u16, collision: &mut bool) -> Option<TranspositionTableResult> {
        let key = self.get_key(hash);
        let index = (hash as usize) % self.table.len();
        let bucket = &self.table[index];
        let mut entry_with_key_present = false;

        for entry in &bucket.entries {
            let entry_data = entry.get_data();
            let mut entry_score = entry_data.score;

            if entry_data.key == key {
                if engine::is_score_near_checkmate(entry_score) {
                    if entry_score > 0 {
                        entry_score -= ply as i16;
                    } else {
                        entry_score += ply as i16;
                    }
                }

                return Some(TranspositionTableResult {
                    key: entry_data.key,
                    score: entry_score,
                    best_move: entry_data.best_move,
                    depth: entry_data.depth,
                    r#type: entry_data.r#type,
                    age: entry_data.age,
                });
            } else if entry_data.key != 0 {
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

        for bucket in self.table.iter().take(BUCKETS_COUNT_TO_CHECK) {
            for entry in &bucket.entries {
                let entry_key_data = entry.key_data.load(Ordering::Relaxed);
                let entry_key = (entry_key_data >> 48) as u16;

                if entry_key != 0 {
                    filled_entries += 1;
                }
            }
        }

        ((filled_entries as f32) / (RESOLUTION as f32)) * 100.0
    }

    /// Increments ann age of all entries stored in the table. If age's value is equal to 31, it gets purged to make more space for a new entries.
    pub fn age_entries(&self) {
        for bucket in &self.table {
            for entry in &bucket.entries {
                let entry_data = entry.get_data();
                if entry_data.depth > 0 {
                    if entry_data.age == 31 {
                        entry.set_data(0, 0, Default::default(), 0, TranspositionTableScoreType::INVALID, 0);
                    } else {
                        entry.set_data(
                            entry_data.key,
                            entry_data.score,
                            entry_data.best_move,
                            entry_data.depth,
                            entry_data.r#type,
                            entry_data.age + 1,
                        );
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
        TranspositionTableBucket { entries: Default::default() }
    }
}

impl TranspositionTableEntry {
    /// Converts `key`, `score`, `best_move`, `depth`, `r#type` and `age` into an atomic word, and stores it.
    pub fn set_data(&self, key: u16, score: i16, best_move: Move, depth: i8, r#type: TranspositionTableScoreType, age: u8) {
        let key_data = 0
            | (key as u64)
            | (((score as u16) as u64) << 16)
            | ((best_move.data as u64) << 32)
            | (((depth as u8) as u64) << 48)
            | ((r#type.bits as u64) << 56)
            | ((age as u64) << 59);

        self.key_data.store(key_data, Ordering::Relaxed);
    }

    /// Loads and parses atomic value into a [TranspositionTableResult] struct.
    pub fn get_data(&self) -> TranspositionTableResult {
        let key_data = self.key_data.load(Ordering::Relaxed);
        TranspositionTableResult {
            key: key_data as u16,
            score: (key_data >> 16) as i16,
            best_move: Move::new_from_raw((key_data >> 32) as u16),
            depth: (key_data >> 48) as i8,
            r#type: TranspositionTableScoreType::from_bits(((key_data >> 56) & 0x7) as u8).unwrap(),
            age: (key_data >> 59) as u8,
        }
    }
}

impl Default for TranspositionTableEntry {
    /// Constructs a default instance of [TranspositionTableEntry] with zeroed elements.
    fn default() -> Self {
        TranspositionTableEntry { key_data: AtomicU64::new(0) }
    }
}

impl Clone for TranspositionTableEntry {
    /// Clones [TranspositionTableEntry] by creating a new atomics (with original values).
    fn clone(&self) -> Self {
        Self {
            key_data: AtomicU64::new(self.key_data.load(Ordering::Relaxed)),
        }
    }
}
