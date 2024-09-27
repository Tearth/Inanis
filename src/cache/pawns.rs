use crate::utils::assert_fast;
use crate::utils::percent;
use std::mem;
use std::sync::atomic::AtomicI16;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use std::u64;

pub struct PawnHashTable {
    pub table: Vec<PawnHashTableEntry>,
}

pub struct PawnHashTableEntry {
    pub key: AtomicU16,
    pub score_opening: AtomicI16,
    pub score_ending: AtomicI16,
}

pub struct PawnHashTableResult {
    pub key: u16,
    pub score_opening: i16,
    pub score_ending: i16,
}

impl PawnHashTable {
    /// Constructs a new instance of [PawnHashTable] by allocating `size` bytes of memory.
    pub fn new(size: usize) -> Self {
        const SIZE: usize = mem::size_of::<PawnHashTableEntry>();
        let mut hashtable = Self { table: Vec::with_capacity(size / SIZE) };

        if size != 0 {
            hashtable.table.resize_with(hashtable.table.capacity(), Default::default);
        }

        hashtable
    }

    /// Adds a new entry (storing the key, `score_opening` and `score_ending`) using `hash` to calculate an index.
    pub fn add(&self, hash: u64, score_opening: i16, score_ending: i16) {
        let key = self.get_key(hash);
        let index = self.get_index(hash);
        assert_fast!(index < self.table.len());

        self.table[index].set_data(key, score_opening, score_ending);
    }

    /// Gets a wanted entry using `hash` to calculate an index. Returns [None] if `hash` is incompatible with the stored key.
    pub fn get(&self, hash: u64) -> Option<PawnHashTableResult> {
        let index = self.get_index(hash);
        assert_fast!(index < self.table.len());

        let entry = &self.table[index];
        let entry_data = entry.get_data();

        if entry_data.key == self.get_key(hash) {
            return Some(entry_data);
        }

        None
    }

    /// Calculates an approximate percentage usage of the table, based on the first `resolution` entries.
    pub fn get_usage(&self, resolution: usize) -> f32 {
        let mut filled_entries = 0;
        for entry in self.table.iter().take(resolution) {
            let entry_data = entry.get_data();
            if entry_data.key != 0 {
                filled_entries += 1;
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

impl PawnHashTableEntry {
    /// Loads and parses atomic value into a [PawnHashTableResult] struct.
    pub fn get_data(&self) -> PawnHashTableResult {
        let key = self.key.load(Ordering::Relaxed);
        let score_opening = self.score_opening.load(Ordering::Relaxed);
        let score_ending = self.score_ending.load(Ordering::Relaxed);

        PawnHashTableResult::new(key, score_opening, score_ending)
    }

    /// Converts `key`, `score_opening` and `score_ending` into an atomic word, and stores it.
    pub fn set_data(&self, key: u16, score_opening: i16, score_ending: i16) {
        self.key.store(key, Ordering::Relaxed);
        self.score_opening.store(score_opening, Ordering::Relaxed);
        self.score_ending.store(score_ending, Ordering::Relaxed);
    }
}

impl Default for PawnHashTableEntry {
    /// Constructs a default instance of [PawnHashTableEntry] with zeroed elements.
    fn default() -> Self {
        PawnHashTableEntry { key: AtomicU16::new(0), score_opening: AtomicI16::new(0), score_ending: AtomicI16::new(0) }
    }
}

impl PawnHashTableResult {
    /// Constructs a new instance of [PawnHashTableResult] with stored `key`, `score_opening` and `score_ending`.
    pub fn new(key: u16, score_opening: i16, score_ending: i16) -> Self {
        Self { key, score_opening, score_ending }
    }
}
