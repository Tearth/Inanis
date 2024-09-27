use crate::engine;
use crate::state::movescan::Move;
use crate::state::representation::Board;
use crate::utils::assert_fast;
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
    pub table: Vec<TranspositionTableBucket>,
}

#[repr(align(64))]
#[derive(Default)]
pub struct TranspositionTableBucket {
    pub entries: [TranspositionTableEntry; BUCKET_SLOTS],
}

#[derive(Default)]
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
        const BUCKET_SIZE: usize = mem::size_of::<TranspositionTableBucket>();
        let mut hashtable = Self { table: Vec::with_capacity(size / BUCKET_SIZE) };

        if size != 0 {
            hashtable.table.resize_with(hashtable.table.capacity(), Default::default);
        }

        hashtable
    }

    /// Adds a new entry (storing the key, `score`, `best_move`, `depth`, `ply`, `r#type` and `age`) using `hash` to calculate an index of the bucket.
    /// Replacement strategy considers a few elements to optimize memory usage and prioritizes slots to replace as follows:
    ///  - empty slots or slots with the same key as the new entry
    ///  - slots with the smallest depth (if there are some old entries, prioritize them)
    ///
    /// This function takes care of converting mate `score` using passed `ply`.
    pub fn add(&self, hash: u64, mut score: i16, best_move: Move, depth: i8, ply: u16, r#type: u8, age: u8) {
        assert_fast!(r#type == 1 || r#type == 2 || r#type == 4);

        let key = self.get_key(hash);
        let index = self.get_index(hash);

        assert_fast!(index < self.table.len());
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

        assert_fast!(desired_index < bucket.entries.len());
        bucket.entries[desired_index].set_data(key, score, best_move, depth, r#type, age);
    }

    /// Gets a wanted entry using `hash` to calculate an index of the bucket. This function takes care of converting
    /// mate `score` using passed `ply`. Returns [None] if `hash` is incompatible with the stored key.
    pub fn get(&self, hash: u64, ply: u16) -> Option<TranspositionTableResult> {
        let key = self.get_key(hash);
        let index = self.get_index(hash);

        assert_fast!(index < self.table.len());
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

    /// Prefetches an entry using `hash` to calculate an index of the bucket. This function should be called early enough, so CPU has
    /// the time to transfer data from the memory into cache.
    pub fn prefetch(&self, hash: u64) {
        unsafe {
            let index = self.get_index(hash);
            let addr = self.table.as_ptr().add(index) as *const i8;

            #[cfg(target_arch = "x86")]
            std::arch::x86::_mm_prefetch::<{ std::arch::x86::_MM_HINT_T0 }>(addr);

            #[cfg(target_arch = "x86_64")]
            std::arch::x86_64::_mm_prefetch::<{ std::arch::x86_64::_MM_HINT_T0 }>(addr);

            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            std::arch::asm!("prfm PSTL1KEEP, [{}]", in(reg) addr);
        }
    }

    /// Gets an entry's best move using `hash` to calculate an index of the bucket.
    /// Returns [None] if `hash` is incompatible with the stored key.
    pub fn get_best_move(&self, hash: u64) -> Option<Move> {
        self.get(hash, 0).map(|entry| entry.best_move)
    }

    /// Retrieves PV line from the transposition table, using `board` position and the current `ply`.
    pub fn get_pv_line(&self, board: &mut Board, ply: i8) -> Vec<Move> {
        if ply >= engine::MAX_DEPTH {
            return Vec::new();
        }

        let mut pv_line = Vec::new();
        match self.get(board.state.hash, 0) {
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

    /// Calculates a key for the `hash` by taking first 16 bits of it.
    fn get_key(&self, hash: u64) -> u16 {
        hash as u16
    }

    /// Calculates an index for the `hash`.
    fn get_index(&self, hash: u64) -> usize {
        (((hash as u128).wrapping_mul(self.table.len() as u128)) >> 64) as usize
    }
}

impl TranspositionTableEntry {
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

    /// Converts `key`, `score`, `best_move`, `depth`, `r#type` and `age` into an atomic word, and stores it.
    pub fn set_data(&self, key: u16, score: i16, best_move: Move, depth: i8, r#type: u8, age: u8) {
        assert_fast!(r#type == 1 || r#type == 2 || r#type == 4);

        let key_data = 0
            | (key as u64)
            | (((score as u16) as u64) << 16)
            | ((best_move.data as u64) << 32)
            | (((depth as u8) as u64) << 48)
            | ((r#type as u64) << 56)
            | ((age as u64) << 59);

        self.key_data.store(key_data, Ordering::Relaxed);
    }
}

impl TranspositionTableResult {
    /// Constructs a new instance of [TranspositionTableResult] with stored `key`, `score`, `best_move`, `depth`, `r#type` and `age`.
    pub fn new(key: u16, score: i16, best_move: Move, depth: i8, r#type: u8, age: u8) -> Self {
        Self { key, score, best_move, depth, r#type, age }
    }
}
