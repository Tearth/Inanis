use std::mem;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::u64;

const BUCKET_SLOTS: usize = 4;

pub struct PerftHashTable {
    pub table: Vec<PerftHashTableBucket>,
}

#[derive(Clone)]
#[repr(align(64))]
pub struct PerftHashTableBucket {
    pub entries: [PerftHashTableEntry; BUCKET_SLOTS],
}

pub struct PerftHashTableEntry {
    pub key: AtomicU64,
    pub data: AtomicU64,
}

pub struct PerftHashTableResult {
    pub leafs_count: u64,
}

impl PerftHashTable {
    /// Constructs a new instance of [PerftHashTable] by allocating `size` bytes of memory.
    pub fn new(size: usize) -> Self {
        let bucket_size = mem::size_of::<PerftHashTableBucket>();
        let aligned_size = 1 << (63 - size.leading_zeros());

        let mut hashtable = Self {
            table: Vec::with_capacity(aligned_size / bucket_size),
        };

        if aligned_size != 0 {
            hashtable.table.resize(hashtable.table.capacity(), Default::default());
        }

        hashtable
    }

    /// Adds a new entry (storing `hash`, `depth` and `leafs_count`) using `hash & (self.table.len() - 1)` formula to calculate an index of the bucket.
    pub fn add(&self, hash: u64, depth: u8, leafs_count: u64) {
        let index = self.get_index(hash);
        let bucket = &self.table[index];

        let mut smallest_depth = u8::MAX;
        let mut smallest_depth_index = 0;

        for (entry_index, entry) in bucket.entries.iter().enumerate() {
            let entry_key = entry.key.load(Ordering::Relaxed);
            let entry_data = entry.data.load(Ordering::Relaxed);
            let entry_depth = ((entry_key ^ entry_data) as u8) & 0xf;

            if entry_depth < smallest_depth {
                smallest_depth = entry_depth;
                smallest_depth_index = entry_index;
            }
        }

        let key = (hash & !0xf) | (depth as u64);
        let data = leafs_count;

        bucket.entries[smallest_depth_index].key.store(key ^ data, Ordering::Relaxed);
        bucket.entries[smallest_depth_index].data.store(data, Ordering::Relaxed);
    }

    /// Gets a wanted entry from the specified `depth` using `hash & (self.table.len() - 1)` formula to calculate an index of the bucket.
    /// Returns [None] if `hash` is incompatible with the stored key.
    pub fn get(&self, hash: u64, depth: u8) -> Option<PerftHashTableResult> {
        let index = self.get_index(hash);
        let bucket = &self.table[index];

        for entry in &bucket.entries {
            let entry_key = entry.key.load(Ordering::Relaxed);
            let entry_data = entry.data.load(Ordering::Relaxed);
            let key = (hash & !0xf) | (depth as u64);

            if (entry_key ^ entry_data) == key {
                return Some(PerftHashTableResult::new(entry_data));
            }
        }

        None
    }

    /// Calculates an approximate percentage usage of the table, based on the first `resolution` entries.
    pub fn get_usage(&self, resolution: usize) -> f32 {
        let buckets_count_to_check: usize = resolution / BUCKET_SLOTS;
        let mut filled_entries = 0;

        for bucket in self.table.iter().take(buckets_count_to_check) {
            for entry in &bucket.entries {
                if entry.key.load(Ordering::Relaxed) != 0 && entry.data.load(Ordering::Relaxed) != 0 {
                    filled_entries += 1;
                }
            }
        }

        ((filled_entries as f32) / (resolution as f32)) * 100.0
    }

    fn get_index(&self, hash: u64) -> usize {
        (hash as usize) & (self.table.len() - 1)
    }
}

impl Default for PerftHashTableBucket {
    /// Constructs a default instance of [PerftHashTableBucket] with zeroed elements.
    fn default() -> Self {
        PerftHashTableBucket { entries: Default::default() }
    }
}

impl Default for PerftHashTableEntry {
    /// Constructs a default instance of [PerftHashTableEntry] with zeroed elements.
    fn default() -> Self {
        Self {
            key: AtomicU64::new(0),
            data: AtomicU64::new(0),
        }
    }
}

impl PerftHashTableResult {
    /// Constructs a new instance of [PerftHashTableResult] with `leafs_count`.
    pub fn new(leafs_count: u64) -> Self {
        Self { leafs_count }
    }
}

impl Clone for PerftHashTableEntry {
    /// Clones [PerftHashTableEntry] by creating a new atomics (with the original values).
    fn clone(&self) -> Self {
        Self {
            key: AtomicU64::new(self.key.load(Ordering::Relaxed)),
            data: AtomicU64::new(self.data.load(Ordering::Relaxed)),
        }
    }
}
