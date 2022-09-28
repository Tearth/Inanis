use std::cmp;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

pub struct HistoryTable {
    pub table: [[HistoryTableEntry; 64]; 64],
    pub max: AtomicU32,
}

pub struct HistoryTableEntry {
    pub data: AtomicU32,
}

pub struct HistoryTableResult {
    pub value: u32,
}

impl HistoryTable {
    /// Increases `[from][to]` history slot value based on `depth`.
    pub fn add(&self, from: u8, to: u8, depth: u8) {
        let entry = &self.table[from as usize][to as usize];
        let entry_data = entry.get_data();
        let updated_value = entry_data.value + (depth as u32).pow(2);

        let max = self.max.load(Ordering::Relaxed);
        self.max.store(cmp::max(max, updated_value), Ordering::Relaxed);

        entry.set_data(updated_value);
    }

    /// Gets `[from][to]` history slot value, relative to `max`.
    pub fn get(&self, from: u8, to: u8, max: u8) -> u8 {
        let entry = &self.table[from as usize][to as usize];
        let entry_data = entry.get_data();
        let max_value = self.max.load(Ordering::Relaxed);

        // Integer ceiling: https://stackoverflow.com/a/2745086
        ((entry_data.value * (max as u32) + max_value - 1) / max_value) as u8
    }

    /// Ages all values in the history table by performing a square root operation and ceiling.
    pub fn age_values(&self) {
        for row in &self.table {
            for entry in row {
                let entry_data = entry.get_data();
                let value_aged = self.age_value(entry_data.value);

                entry.set_data(value_aged);
            }
        }

        let max = self.max.load(Ordering::Relaxed);
        let max_aged = self.age_value(max);
        self.max.store(max_aged, Ordering::Relaxed);
    }

    /// Ages a single value by performing a square root operation and ceiling.
    fn age_value(&self, value: u32) -> u32 {
        (value as f32).sqrt().ceil() as u32
    }
}

impl Default for HistoryTable {
    /// Constructs a default instance of [HistoryTable] with zeroed elements.
    fn default() -> Self {
        const INIT_1: HistoryTableEntry = HistoryTableEntry::new_const();
        const INIT_2: [HistoryTableEntry; 64] = [INIT_1; 64];

        HistoryTable { table: [INIT_2; 64], max: AtomicU32::new(1) }
    }
}

impl Clone for HistoryTable {
    /// Clones [HistoryTable] by creating a new atomics (with the original values).
    fn clone(&self) -> Self {
        Self { table: self.table.clone(), max: AtomicU32::new(self.max.load(Ordering::Relaxed)) }
    }
}

impl HistoryTableEntry {
    /// Constructs a new instance of [HistoryTableEntry] with zeroed values.
    pub const fn new_const() -> Self {
        Self { data: AtomicU32::new(0) }
    }

    /// Converts `value` into an atomic word, and stores it.
    pub fn set_data(&self, value: u32) {
        self.data.store(value, Ordering::Relaxed);
    }

    /// Loads and parses atomic value into a [HistoryTableEntry] struct.
    pub fn get_data(&self) -> HistoryTableResult {
        HistoryTableResult { value: self.data.load(Ordering::Relaxed) }
    }
}

impl Default for HistoryTableEntry {
    /// Constructs a default instance of [HistoryTableEntry] with zeroed elements.
    fn default() -> Self {
        Self { data: AtomicU32::new(0) }
    }
}

impl Clone for HistoryTableEntry {
    /// Clones [HistoryTableEntry] by creating a new atomic (with the original value).
    fn clone(&self) -> Self {
        Self { data: AtomicU32::new(self.data.load(Ordering::Relaxed)) }
    }
}
