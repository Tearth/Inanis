use crate::engine;
use crate::state::movescan::Move;
use crate::state::representation::Board;
use crate::utils::percent;
use std::mem;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::u64;

const BUCKET_SLOTS: usize = 8;

#[allow(non_snake_case)]
pub mod TranspositionTableScoreType {
    pub const INVALID: u8 = 0;
    pub const EXACT_SCORE: u8 = 1;
    pub const UPPER_BOUND: u8 = 2;
    pub const LOWER_BOUND: u8 = 4;
}

pub struct TranspositionTable {
    table: Vec<TranspositionTableBucket>,
}

#[derive(Clone)]
#[repr(align(64))]
pub struct TranspositionTableBucket {
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
    pub r#type: u8,
    pub age: u8,
}

impl TranspositionTable {
    /// Constructs a new instance of [TranspositionTable] by allocating `size` bytes of memory.
    pub fn new(size: usize) -> Self {
        let bucket_size = mem::size_of::<TranspositionTableBucket>();
        let aligned_size = if size != 0 { 1 << (63 - size.leading_zeros()) } else { 0 };
        let mut hashtable = Self { table: Vec::with_capacity(aligned_size / bucket_size) };

        if aligned_size != 0 {
            hashtable.table.resize(hashtable.table.capacity(), Default::default());
        }

        hashtable
    }

    /// Adds a new entry (storing the key, `score`, `best_move`, `depth`, `ply`, `score_type` and `age`) using `hash & (self.table.len() - 1)` formula
    /// to calculate an index of the bucket. Replacement strategy considers a few elements to optimize memory usage and prioritizes slots to replace as follows:
    ///  - empty slots or slots with the same key as the new entry
    ///  - slots with the smallest depth (if there are some old entries, prioritize them)
    ///
    /// This function takes care of converting mate `score` using passed `ply`.
    pub fn add(&self, hash: u64, mut score: i16, best_move: Move, depth: i8, ply: u16, score_type: u8, age: u8) {
        let key = self.get_key(hash);
        let index = self.get_index(hash);
        let bucket = &self.table[index];

        let mut smallest_depth = i8::MAX;
        let mut desired_index = usize::MAX;
        let mut found_old_entry = false;

        for (entry_index, entry) in bucket.entries.iter().enumerate() {
            let entry_data = entry.get_data();

            if entry_data.depth == 0 || entry_data.key == key {
                desired_index = entry_index;
                break;
            }

            if entry_data.age != age {
                if found_old_entry {
                    if entry_data.depth < smallest_depth {
                        desired_index = entry_index;
                        smallest_depth = entry_data.depth;
                    }
                } else {
                    desired_index = entry_index;
                    smallest_depth = entry_data.depth;
                    found_old_entry = true;
                }

                continue;
            }

            if !found_old_entry && entry_data.depth < smallest_depth {
                smallest_depth = entry_data.depth;
                desired_index = entry_index;
                continue;
            }
        }

        if engine::is_score_near_checkmate(score) {
            if score > 0 {
                score += ply as i16;
            } else {
                score -= ply as i16;
            }
        }

        bucket.entries[desired_index].set_data(key, score, best_move, depth, score_type, age);
    }

    /// Gets a wanted entry using `hash & (self.table.len() - 1)` formula to calculate an index of the bucket. This function takes care of converting
    /// mate `score` using passed `ply`. Returns [None] if `hash` is incompatible with the stored key.
    pub fn get(&self, hash: u64, ply: u16) -> Option<TranspositionTableResult> {
        let key = self.get_key(hash);
        let index = self.get_index(hash);
        let bucket = &self.table[index];

        for entry in &bucket.entries {
            let entry_data = entry.get_data();
            if entry_data.key == key {
                let entry_score = if engine::is_score_near_checkmate(entry_data.score) {
                    if entry_data.score > 0 {
                        entry_data.score - (ply as i16)
                    } else {
                        entry_data.score + (ply as i16)
                    }
                } else {
                    entry_data.score
                };

                return Some(TranspositionTableResult::new(
                    entry_data.key,
                    entry_score,
                    entry_data.best_move,
                    entry_data.depth,
                    entry_data.r#type,
                    entry_data.age,
                ));
            }
        }

        None
    }

    /// Gets an entry's best move using `hash & (self.table.len() - 1)` formula to calculate an index of the bucket.
    /// Returns [None] if `hash` is incompatible with the stored key.
    pub fn get_best_move(&self, hash: u64) -> Option<Move> {
        let entry = self.get(hash, 0);
        entry.map(|entry| entry.best_move)
    }

    /// Retrieves PV line from the transposition table, using `board` position and the current `ply`.
    pub fn get_pv_line(&self, board: &mut Board, ply: i8) -> Vec<Move> {
        if ply >= engine::MAX_DEPTH {
            return Vec::new();
        }

        let mut pv_line = Vec::new();
        match self.get(board.hash, 0) {
            Some(entry) => {
                if entry.r#type != TranspositionTableScoreType::EXACT_SCORE {
                    return Vec::new();
                }

                if entry.best_move.is_legal(board) {
                    board.make_move(entry.best_move);
                    if !board.is_king_checked(board.active_color ^ 1) {
                        pv_line.push(entry.best_move);
                        pv_line.append(&mut self.get_pv_line(board, ply + 1));
                    }
                    board.undo_move(entry.best_move);
                }
            }
            None => {
                return Vec::new();
            }
        }

        // Remove endless repetitions from PV line
        if pv_line.len() > 8 {
            if pv_line[0] == pv_line[4] && pv_line[4] == pv_line[8] {
                pv_line = pv_line[0..1].to_vec();
            }
        }

        pv_line
    }

    /// Calculates an approximate percentage usage of the table, based on the first `resolution` entries.
    pub fn get_usage(&self, resolution: usize) -> f32 {
        let buckets_count_to_check: usize = resolution / BUCKET_SLOTS;
        let mut filled_entries = 0;

        for bucket in self.table.iter().take(buckets_count_to_check) {
            for entry in &bucket.entries {
                let entry_key_data = entry.key_data.load(Ordering::Relaxed);
                let entry_key = (entry_key_data >> 48) as u16;

                if entry_key != 0 {
                    filled_entries += 1;
                }
            }
        }

        percent!(filled_entries, resolution)
    }

    /// Calculates a key for the `hash` by taking the last 16 bits of it.
    fn get_key(&self, hash: u64) -> u16 {
        (hash >> 48) as u16
    }

    /// Calculates an index for the `hash`.
    fn get_index(&self, hash: u64) -> usize {
        (hash as usize) & (self.table.len() - 1)
    }
}

impl Default for TranspositionTableBucket {
    /// Constructs a default instance of [TranspositionTableBucket] with zeroed elements.
    fn default() -> Self {
        Self { entries: Default::default() }
    }
}

impl TranspositionTableEntry {
    /// Converts `key`, `score`, `best_move`, `depth`, `r#type` and `age` into an atomic word, and stores it.
    pub fn set_data(&self, key: u16, score: i16, best_move: Move, depth: i8, r#type: u8, age: u8) {
        let key_data = 0
            | (key as u64)
            | (((score as u16) as u64) << 16)
            | ((best_move.data as u64) << 32)
            | (((depth as u8) as u64) << 48)
            | ((r#type as u64) << 56)
            | ((age as u64) << 59);

        self.key_data.store(key_data, Ordering::Relaxed);
    }

    /// Loads and parses atomic value into a [TranspositionTableResult] struct.
    pub fn get_data(&self) -> TranspositionTableResult {
        let key_data = self.key_data.load(Ordering::Relaxed);

        let key = key_data as u16;
        let score = (key_data >> 16) as i16;
        let best_move = Move::new_from_raw((key_data >> 32) as u16);
        let depth = (key_data >> 48) as i8;
        let r#type = ((key_data >> 56) & 0x7) as u8;
        let age = (key_data >> 59) as u8;

        TranspositionTableResult::new(key, score, best_move, depth, r#type, age)
    }
}

impl Default for TranspositionTableEntry {
    /// Constructs a default instance of [TranspositionTableEntry] with zeroed elements.
    fn default() -> Self {
        TranspositionTableEntry { key_data: AtomicU64::new(0) }
    }
}

impl Clone for TranspositionTableEntry {
    /// Clones [TranspositionTableEntry] by creating a new atomics (with the original values).
    fn clone(&self) -> Self {
        Self { key_data: AtomicU64::new(self.key_data.load(Ordering::Relaxed)) }
    }
}

impl TranspositionTableResult {
    /// Constructs a new instance of [TranspositionTableResult] with stored `key`, `score`, `best_move`, `depth`, `r#type` and `age`.
    pub fn new(key: u16, score: i16, best_move: Move, depth: i8, r#type: u8, age: u8) -> Self {
        Self { key, score, best_move, depth, r#type, age }
    }
}
