#[inline(always)]
pub fn get_lsb(value: u64) -> u64 {
    value & value.wrapping_neg()
}

#[inline(always)]
pub fn pop_lsb(value: u64) -> u64 {
    value & (value - 1)
}

#[inline(always)]
pub fn bit_count(value: u64) -> u8 {
    value.count_ones() as u8
}

#[inline(always)]
pub fn bit_scan(value: u64) -> u8 {
    value.trailing_zeros() as u8
}
