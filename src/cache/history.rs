use crate::utils::divceil::DivCeil;
use std::cmp;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

const AGING_DIVISOR: u32 = 64;

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
    pub fn add(&self, from: usize, to: usize, depth: u8) {
        let entry = &self.table[from][to];
        let entry_data = entry.get_data();
        let updated_value = entry_data.value + (depth as u32).pow(2);

        let max = self.max.load(Ordering::Relaxed);
        self.max.store(cmp::max(max, updated_value), Ordering::Relaxed);

        entry.set_data(updated_value);
    }

    pub fn punish(&self, from: usize, to: usize, depth: u8) {
        let entry = &self.table[from][to];
        let entry_data = entry.get_data();

        let value = depth as u32;
        let updated_value = if value > entry_data.value { 0 } else { entry_data.value - value };

        entry.set_data(updated_value);
    }

    /// Gets `[from][to]` history slot value, relative to `max`.
    pub fn get(&self, from: usize, to: usize, max: u8) -> u8 {
        let entry = &self.table[from][to];
        let entry_data = entry.get_data();
        let max_value = self.max.load(Ordering::Relaxed);

        (entry_data.value * (max as u32)).div_ceil_stable(max_value) as u8
    }

    /// Ages all values in the history table by dividing them by the [AGING_DIVISOR].
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

    /// Ages a single value by dividing value by the [AGING_DIVISOR].
    fn age_value(&self, value: u32) -> u32 {
        value.div_ceil_stable(AGING_DIVISOR)
    }
}

impl Default for HistoryTable {
    /// Constructs a default instance of [HistoryTable] with zeroed elements (except `max`).
    fn default() -> Self {
        const INIT_1: HistoryTableEntry = HistoryTableEntry::new_const();
        const INIT_2: [HistoryTableEntry; 64] = [INIT_1; 64];

        HistoryTable { table: [INIT_2; 64], max: AtomicU32::new(1) }
    }
}

impl Clone for HistoryTable {
    /// Clones [HistoryTable] by creating new atomics (with the original values).
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
        HistoryTableResult::new(self.data.load(Ordering::Relaxed))
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

impl HistoryTableResult {
    /// Constructs a new instance of [HistoryTableResult] with stored `value`.
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}
