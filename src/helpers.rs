pub fn get_lsb(value: u64) -> u64 {
    ((value as i64) & -(value as i64)) as u64
}

pub fn pop_lsb(value: u64) -> u64 {
    value & (value - 1)
}

pub fn bit_count(value: u64) -> u32 {
    value.count_ones()
}

pub fn bit_scan(value: u64) -> u32 {
    value.trailing_zeros()
}