use crate::utils::divceil::DivCeil;
use std::cmp;

const AGING_DIVISOR: u32 = 16;

pub struct HistoryTable {
    pub table: Box<[[HistoryTableEntry; 64]; 64]>,
    pub max: u32,
}

pub struct HistoryTableEntry {
    pub data: u32,
}

impl HistoryTable {
    /// Increases `[from][to]` history slot value based on `depth`.
    pub fn add(&mut self, from: usize, to: usize, depth: u8) {
        let entry = &mut self.table[from][to];
        let updated_value = entry.data + (depth as u32).pow(2);
        self.max = cmp::max(self.max, updated_value);

        entry.data = updated_value;
    }

    /// Punishes `[from][to]` history slot value based on `depth`.
    pub fn punish(&mut self, from: usize, to: usize, depth: u8) {
        let entry = &mut self.table[from][to];

        let value = depth as u32;
        let updated_value = if value > entry.data { 0 } else { entry.data - value };

        entry.data = updated_value;
    }

    /// Gets `[from][to]` history slot value, relative to `max`.
    pub fn get(&self, from: usize, to: usize, max: u8) -> u8 {
        let entry = &self.table[from][to];
        let max_value = self.max;

        (entry.data * (max as u32)).div_ceil_stable(max_value) as u8
    }

    /// Ages all values in the history table by dividing them by the [AGING_DIVISOR].
    pub fn age_values(&mut self) {
        for row in self.table.iter_mut() {
            for entry in row {
                entry.data = entry.data.div_ceil_stable(AGING_DIVISOR);
            }
        }

        self.max = self.age_value(self.max);
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

        HistoryTable { table: Box::new([INIT_2; 64]), max: 1 }
    }
}

impl HistoryTableEntry {
    /// Constructs a new instance of [HistoryTableEntry] with zeroed values.
    pub const fn new_const() -> Self {
        Self { data: 0 }
    }
}

impl Default for HistoryTableEntry {
    /// Constructs a default instance of [HistoryTableEntry] with zeroed elements.
    fn default() -> Self {
        Self { data: 0 }
    }
}
