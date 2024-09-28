use crate::utils::assert_fast;
use crate::utils::divceil::DivCeil;
use std::alloc;
use std::alloc::Layout;
use std::cmp;
use std::mem;

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
        assert_fast!(from < 64);
        assert_fast!(to < 64);

        let entry = &mut self.table[from][to];
        let value = (depth as u32).pow(2);
        let updated_value = entry.data + value;
        self.max = cmp::max(self.max, updated_value);

        entry.data = updated_value;
    }

    /// Punishes `[from][to]` history slot value based on `depth`.
    pub fn punish(&mut self, from: usize, to: usize, depth: u8) {
        assert_fast!(from < 64);
        assert_fast!(to < 64);

        let entry = &mut self.table[from][to];
        let value = depth as u32;
        let updated_value = match value <= entry.data {
            true => entry.data - value,
            false => 0,
        };

        entry.data = updated_value;
    }

    /// Gets `[from][to]` history slot value, relative to `max`.
    pub fn get(&self, from: usize, to: usize, max: u8) -> u8 {
        assert_fast!(from < 64);
        assert_fast!(to < 64);
        assert_fast!(max > 0);
        assert_fast!(self.max > 0);

        (self.table[from][to].data * (max as u32)).div_ceil_stable(self.max) as u8
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
    /// Constructs a default instance of [HistoryTable] by allocating `64 * 64 * mem::size_of::<HistoryTableEntry>()`
    /// boxed array with zeroed elements.
    fn default() -> Self {
        const SIZE: usize = mem::size_of::<HistoryTableEntry>();
        unsafe {
            let ptr = alloc::alloc_zeroed(Layout::from_size_align(64 * 64 * SIZE, SIZE).unwrap());
            Self { table: Box::from_raw(ptr as *mut [[HistoryTableEntry; 64]; 64]), max: 1 }
        }
    }
}
