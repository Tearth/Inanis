use std::mem;
use std::u64;

pub struct PawnHashTable {
    table: Vec<PawnHashTableEntry>,
    slots: usize,
}

#[derive(Clone, Copy)]
pub struct PawnHashTableEntry {
    pub key: u16,
    pub score: i16,
}

impl PawnHashTable {
    /// Constructs a new instance of [PawnHashTable] by allocating `size` bytes of memory.
    pub fn new(size: usize) -> PawnHashTable {
        let bucket_size = mem::size_of::<PawnHashTableEntry>();
        let buckets = size / bucket_size;
        let mut hashtable = PawnHashTable {
            table: Vec::with_capacity(buckets),
            slots: buckets,
        };

        if size != 0 {
            hashtable.table.resize(hashtable.slots, Default::default());
        }

        hashtable
    }

    /// Adds a new entry (storing key and `score`) using `hash % self.slots` formula to calculate index.
    pub fn add(&mut self, hash: u64, score: i16) {
        self.table[(hash as usize) % self.slots] = PawnHashTableEntry::new(self.get_key(hash), score);
    }

    /// Gets wanted entry using `hash % self.slots` formula to calculate index. Returns [None] if `hash` is incompatible with the stored key.
    pub fn get(&self, hash: u64, collision: &mut bool) -> Option<PawnHashTableEntry> {
        let entry = self.table[(hash as usize) % self.slots];
        if entry.key == self.get_key(hash) {
            return Some(entry);
        }

        if entry.key != 0 {
            *collision = true;
        }

        None
    }

    /// Calculates approximate percentage usage of the table, based on first 10000 entries.
    pub fn get_usage(&self) -> f32 {
        const RESOLUTION: usize = 10000;
        let mut filled_entries = 0;

        for entry_index in 0..RESOLUTION {
            if self.table[entry_index].key != 0 {
                filled_entries += 1;
            }
        }

        ((filled_entries as f32) / (RESOLUTION as f32)) * 100.0
    }

    /// Calculates a key for the `hash` by taking the last 16 bits of it.
    fn get_key(&self, hash: u64) -> u16 {
        (hash >> 48) as u16
    }
}

impl PawnHashTableEntry {
    /// Constructs a new instance of [PawnHashTableEntry] with stored `key` and `score`.
    pub fn new(key: u16, score: i16) -> PawnHashTableEntry {
        PawnHashTableEntry { key, score }
    }
}

impl Default for PawnHashTableEntry {
    /// Constructs a default instance of [PawnHashTableEntry] with zeroed elements.
    fn default() -> Self {
        PawnHashTableEntry::new(0, 0)
    }
}
