use std::mem;
use std::u64;

pub struct PawnsHashTable {
    table: Vec<PawnsHashTableEntry>,
    slots: usize,
}

#[derive(Clone, Copy)]
pub struct PawnsHashTableEntry {
    pub key: u16,
    pub score: i16,
}

impl PawnsHashTable {
    pub fn new(size: usize) -> PawnsHashTable {
        let buckets = size / mem::size_of::<PawnsHashTableEntry>();
        let mut hashtable = PawnsHashTable {
            table: Vec::with_capacity(buckets),
            slots: buckets,
        };

        if size != 0 {
            hashtable.table.resize(hashtable.slots, PawnsHashTableEntry::new(0, 0));
        }

        hashtable
    }

    pub fn add(&mut self, hash: u64, score: i16) {
        self.table[(hash as usize) % self.slots] = PawnsHashTableEntry::new((hash >> 48) as u16, score);
    }

    pub fn get(&self, hash: u64) -> PawnsHashTableEntry {
        self.table[(hash as usize) % self.slots]
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

impl PawnsHashTableEntry {
    pub fn new(key: u16, score: i16) -> PawnsHashTableEntry {
        PawnsHashTableEntry { key, score }
    }
}
