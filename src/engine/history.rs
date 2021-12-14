pub struct HistoryTable {
    pub table: [[u32; 64]; 64],
    pub max: u32,
}

impl HistoryTable {
    pub fn add(&mut self, from: u8, to: u8, depth: u8) {
        self.table[from as usize][to as usize] += (depth as u32) * (depth as u32);
        if self.table[from as usize][to as usize] > self.max {
            self.max = self.table[from as usize][to as usize];
        }
    }

    pub fn get(&self, from: u8, to: u8, max: u8) -> u8 {
        ((self.table[from as usize][to as usize] * (max as u32) + self.max - 1) / self.max) as u8
    }

    pub fn age_values(&mut self) {
        for x in 0..64 {
            for y in 0..64 {
                self.table[x][y] = (self.table[x][y] as f32).sqrt().ceil() as u32;
            }
        }

        self.max = (self.max as f32).sqrt().ceil() as u32;
    }
}

impl Default for HistoryTable {
    fn default() -> Self {
        HistoryTable {
            table: [[0; 64]; 64],
            max: 1,
        }
    }
}
