use std::mem;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::u64;

#[derive(Clone)]
pub struct PawnHashTable {
    pub table: Vec<PawnHashTableEntry>,
}

pub struct PawnHashTableEntry {
    pub key_data: AtomicU32,
}

pub struct PawnHashTableResult {
    pub key: u16,
    pub score: i16,
}

impl PawnHashTable {
    /// Constructs a new instance of [PawnHashTable] by allocating `size` bytes of memory.
    pub fn new(size: usize) -> Self {
        let bucket_size = mem::size_of::<PawnHashTableEntry>();
        let mut hashtable = Self {
            table: Vec::with_capacity(size / bucket_size),
        };

        if size != 0 {
            hashtable.table.resize(hashtable.table.capacity(), Default::default());
        }

        hashtable
    }

    /// Adds a new entry (storing the key and `score`) using `hash % self.table.len()` formula to calculate an index.
    pub fn add(&self, hash: u64, score: i16) {
        let key = self.get_key(hash);
        let index = (hash as usize) % self.table.len();

        self.table[index].set_data(key, score);
    }

    /// Gets a wanted entry using `hash % self.table.len()` formula to calculate an index. Returns [None] if `hash` is incompatible with the stored key.
    pub fn get(&self, hash: u64) -> Option<PawnHashTableResult> {
        let entry = &self.table[(hash as usize) % self.table.len()];
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

        ((filled_entries as f32) / (resolution as f32)) * 100.0
    }

    /// Calculates a key for the `hash` by taking the last 16 bits of it.
    fn get_key(&self, hash: u64) -> u16 {
        (hash >> 48) as u16
    }
}

impl PawnHashTableEntry {
    /// Converts `key` and `score` into an atomic word, and stores it.
    pub fn set_data(&self, key: u16, score: i16) {
        let key_data = (key as u32) | (((score as u16) as u32) << 16);
        self.key_data.store(key_data, Ordering::Relaxed);
    }

    /// Loads and parses atomic value into a [PawnHashTableResult] struct.
    pub fn get_data(&self) -> PawnHashTableResult {
        let key_data = self.key_data.load(Ordering::Relaxed);
        PawnHashTableResult {
            key: key_data as u16,
            score: (key_data >> 16) as i16,
        }
    }
}

impl Default for PawnHashTableEntry {
    /// Constructs a default instance of [PawnHashTableEntry] with zeroed elements.
    fn default() -> Self {
        PawnHashTableEntry { key_data: AtomicU32::new(0) }
    }
}

impl Clone for PawnHashTableEntry {
    /// Clones [PawnHashTableEntry] by creating a new atomic (with the original value).
    fn clone(&self) -> Self {
        Self {
            key_data: AtomicU32::new(self.key_data.load(Ordering::Relaxed)),
        }
    }
}
