use std::cell::UnsafeCell;
use std::mem;
use std::u64;

const BUCKET_SLOTS: usize = 4;

pub struct PerftHashTable {
    table: UnsafeCell<Vec<PerftHashTableBucket>>,
    slots: usize,
}

#[repr(align(64))]
#[derive(Clone, Copy)]
struct PerftHashTableBucket {
    pub entries: [PerftHashTableEntry; BUCKET_SLOTS],
}

#[derive(Clone, Copy)]
pub struct PerftHashTableEntry {
    pub key_and_depth: u64,
    pub leafs_count: u64,
}

impl PerftHashTable {
    /// Constructs a new instance of [PerftHashTable] by allocating `size` bytes of memory.
    pub fn new(size: usize) -> PerftHashTable {
        let bucket_size = mem::size_of::<PerftHashTableBucket>();
        let buckets = size / bucket_size;
        let hashtable = PerftHashTable {
            table: UnsafeCell::new(Vec::with_capacity(buckets)),
            slots: buckets,
        };

        if size != 0 {
            unsafe { (*hashtable.table.get()).resize(hashtable.slots, Default::default()) };
        }

        hashtable
    }

    /// Adds a new entry (storing `hash`, `depth` and `leafs_count`) using `hash % self.slots` formula to calculate index of bucket.
    pub fn add(&self, hash: u64, depth: u8, leafs_count: u64) {
        let mut bucket = unsafe { (*self.table.get())[(hash as usize) % self.slots] };
        let mut smallest_depth = (bucket.entries[0].key_and_depth & 0xf) as u8;
        let mut smallest_depth_index = 0;

        for entry_index in 1..BUCKET_SLOTS {
            let entry_depth = (bucket.entries[entry_index].key_and_depth & 0xf) as u8;
            if entry_depth < smallest_depth {
                smallest_depth = entry_depth;
                smallest_depth_index = entry_index;
            }
        }

        bucket.entries[smallest_depth_index] = PerftHashTableEntry::new(hash, depth, leafs_count);
        unsafe { (*self.table.get())[(hash as usize) % self.slots] = bucket };
    }

    /// Gets wanted entry using `hash % self.slots` formula to calculate index of bucket. Returns [None] if `hash` is incompatible with the stored key.
    pub fn get(&self, hash: u64, depth: u8) -> Option<PerftHashTableEntry> {
        let bucket = unsafe { (*self.table.get())[(hash as usize) % self.slots] };
        for entry_index in 0..BUCKET_SLOTS {
            let entry = bucket.entries[entry_index];
            if entry.key_and_depth == ((hash & !0xf) | (depth as u64)) {
                return Some(entry);
            }
        }

        None
    }

    /// Calculates approximate percentage usage of the table, based on first 10000 entries.
    pub fn get_usage(&self) -> f32 {
        const RESOLUTION: usize = 10000;
        const BUCKETS_COUNT_TO_CHECK: usize = RESOLUTION / BUCKET_SLOTS;
        let mut filled_entries = 0;

        for bucket_index in 0..BUCKETS_COUNT_TO_CHECK {
            for entry_index in 0..BUCKET_SLOTS {
                let entry = unsafe { (*self.table.get())[bucket_index].entries[entry_index] };
                if entry.key_and_depth != 0 && entry.leafs_count != 0 {
                    filled_entries += 1;
                }
            }
        }

        ((filled_entries as f32) / (RESOLUTION as f32)) * 100.0
    }
}

unsafe impl Sync for PerftHashTable {}

impl Default for PerftHashTableBucket {
    /// Constructs a default instance of [PerftHashTableBucket] with zeroed elements.
    fn default() -> Self {
        PerftHashTableBucket {
            entries: [Default::default(); BUCKET_SLOTS],
        }
    }
}

impl PerftHashTableEntry {
    /// Constructs a new instance of [PerftHashTableEntry] with stored `key`, `depth` and `leafs_count`.
    pub fn new(key: u64, depth: u8, leafs_count: u64) -> PerftHashTableEntry {
        PerftHashTableEntry {
            key_and_depth: (key & !0xf) | (depth as u64),
            leafs_count,
        }
    }
}

impl Default for PerftHashTableEntry {
    /// Constructs a default instance of [PerftHashTableEntry] with zeroed elements.
    fn default() -> Self {
        PerftHashTableEntry::new(0, 0, 0)
    }
}
