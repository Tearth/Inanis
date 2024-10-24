use crate::utils::assert_fast;
use crate::utils::percent;
use std::mem;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

const BUCKET_SLOTS: usize = 4;

pub struct PerftHashTable {
    pub table: Vec<PerftHashTableBucket>,
}

#[repr(align(64))]
#[derive(Default)]
pub struct PerftHashTableBucket {
    pub entries: [PerftHashTableEntry; BUCKET_SLOTS],
}

#[derive(Default)]
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
        const BUCKET_SIZE: usize = mem::size_of::<PerftHashTableBucket>();
        let mut hashtable = Self { table: Vec::with_capacity(size / BUCKET_SIZE) };

        if BUCKET_SIZE != 0 {
            hashtable.table.resize_with(hashtable.table.capacity(), PerftHashTableBucket::default);
        }

        hashtable
    }

    /// Adds a new entry (storing `hash`, `depth` and `leafs_count`) using `hash` to calculate an index of the bucket.
    pub fn add(&self, hash: u64, depth: u8, leafs_count: u64) {
        let index = self.get_index(hash);
        assert_fast!(index < self.table.len());

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

        assert_fast!(smallest_depth_index < BUCKET_SLOTS);
        bucket.entries[smallest_depth_index].key.store(key ^ data, Ordering::Relaxed);
        bucket.entries[smallest_depth_index].data.store(data, Ordering::Relaxed);
    }

    /// Gets a wanted entry from the specified `depth` using `hash` to calculate an index of the bucket.
    /// Returns [None] if `hash` is incompatible with the stored key.
    pub fn get(&self, hash: u64, depth: u8) -> Option<PerftHashTableResult> {
        let index = self.get_index(hash);
        assert_fast!(index < self.table.len());

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

        percent!(filled_entries, resolution)
    }

    /// Calculates an index for the `hash`.
    fn get_index(&self, hash: u64) -> usize {
        (((hash as u128).wrapping_mul(self.table.len() as u128)) >> 64) as usize
    }
}

impl PerftHashTableResult {
    /// Constructs a new instance of [PerftHashTableResult] with stored `leafs_count`.
    pub fn new(leafs_count: u64) -> Self {
        Self { leafs_count }
    }
}
