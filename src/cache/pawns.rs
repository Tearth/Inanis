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
    pub fn new(size: usize) -> PawnHashTable {
        let bucket_size = mem::size_of::<PawnHashTableEntry>();
        let buckets = size / bucket_size;
        let mut hashtable = PawnHashTable {
            table: Vec::with_capacity(buckets),
            slots: buckets,
        };

        if size != 0 {
            hashtable.table.resize(hashtable.slots, PawnHashTableEntry::new(0, 0));
        }

        hashtable
    }

    pub fn add(&mut self, hash: u64, score: i16) {
        self.table[(hash as usize) % self.slots] = PawnHashTableEntry::new((hash >> 48) as u16, score);
    }

    pub fn get(&self, hash: u64, collision: &mut bool) -> Option<PawnHashTableEntry> {
        let entry = self.table[(hash as usize) % self.slots];
        if entry.key == (hash >> 48) as u16 {
            return Some(entry);
        }

        if entry.key != 0 {
            *collision = true;
        }

        None
    }

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
}

impl PawnHashTableEntry {
    pub fn new(key: u16, score: i16) -> PawnHashTableEntry {
        PawnHashTableEntry { key, score }
    }
}
