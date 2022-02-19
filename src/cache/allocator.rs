use std::cmp;

pub struct AllocationResult {
    pub transposition_table_size: usize,
    pub pawn_hashtable_size: usize,
}

/// Calculates optimal size for hashtables based on `total_size` megabytes of available memory.
/// Minimal size for each of them is 1 MB.
pub fn get_allocation(total_size: usize) -> AllocationResult {
    let pawn_hashtable_size = cmp::max(1, total_size / 128);
    let transposition_table_size = total_size - pawn_hashtable_size;

    AllocationResult {
        transposition_table_size,
        pawn_hashtable_size,
    }
}
