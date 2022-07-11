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
    /// Increases `[from][to]` history slot based on `depth` value.
    pub fn add(&self, from: u8, to: u8, depth: u8) {
        let entry = &self.table[from as usize][to as usize];
        let entry_data = entry.get_data();

        let updated_value = entry_data.value + ((depth as u32) * (depth as u32));
        entry.set_data(updated_value);

        if updated_value > self.max.load(Ordering::Relaxed) {
            self.max.store(updated_value, Ordering::Relaxed);
        }
    }

    /// Gets `[from][to]` history slot value, relative to `max`.
    pub fn get(&self, from: u8, to: u8, max: u8) -> u8 {
        let entry = &self.table[from as usize][to as usize];
        let entry_data = entry.get_data();

        let max_value = self.max.load(Ordering::Relaxed);
        ((entry_data.value * (max as u32) + max_value - 1) / max_value) as u8
    }

    /// Ages all values in the history table by performing square root operation.
    pub fn age_values(&self) {
        for x in 0..64 {
            for y in 0..64 {
                let entry = &self.table[x][y];
                let entry_data = entry.get_data();

                self.table[x][y].set_data((entry_data.value as f32).sqrt().ceil() as u32);
            }
        }

        let updated_max = (self.max.load(Ordering::Relaxed) as f32).sqrt().ceil() as u32;
        self.max.store(updated_max, Ordering::Relaxed);
    }
}

impl Default for HistoryTable {
    /// Constructs a default instance of [HistoryTable] with zeroed elements.
    fn default() -> Self {
        const INIT_1: HistoryTableEntry = HistoryTableEntry::new_const();
        const INIT_2: [HistoryTableEntry; 64] = [INIT_1; 64];

        HistoryTable {
            table: [INIT_2; 64],
            max: AtomicU32::new(1),
        }
    }
}

impl Clone for HistoryTable {
    /// Clones [HistoryTable] by creating a new atomics (with original values).
    fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            max: AtomicU32::new(self.max.load(Ordering::Relaxed)),
        }
    }
}

impl HistoryTableEntry {
    /// Constructs a new instance of [HistoryTableEntry] with zeroed values.
    pub const fn new_const() -> Self {
        Self { data: AtomicU32::new(0) }
    }

    /// Converts `r#move` into an atomic word, and stores it.
    pub fn set_data(&self, value: u32) {
        self.data.store(value, Ordering::Relaxed);
    }

    /// Loads and parses atomic value into a [KillersTableEntry] struct.
    pub fn get_data(&self) -> HistoryTableResult {
        let data = self.data.load(Ordering::Relaxed);
        HistoryTableResult { value: data }
    }
}

impl Default for HistoryTableEntry {
    /// Constructs a default instance of [HistoryTableEntry] with zeroed elements.
    fn default() -> Self {
        Self { data: AtomicU32::new(0) }
    }
}

impl Clone for HistoryTableEntry {
    /// Clones [HistoryTableEntry] by creating a new atomics (with original values).
    fn clone(&self) -> Self {
        Self {
            data: AtomicU32::new(self.data.load(Ordering::Relaxed)),
        }
    }
}
