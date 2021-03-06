use std::cmp;

const PAWN_HASHTABLE_SIZE_FRACTION: usize = 64;

pub struct AllocationResult {
    pub transposition_table_size: usize,
    pub pawn_hashtable_size: usize,
}

/// Calculates optimal size for the hashtables based on `total_size` megabytes of available memory. For every [PAWN_HASHTABLE_SIZE_FRACTION] megabytes
/// of transposition table, there will be 1 MB of pawn hashtable. Minimal size for each of them is 1 MB.
pub fn get_allocation(total_size: usize) -> AllocationResult {
    let pawn_hashtable_size = 1 + total_size / PAWN_HASHTABLE_SIZE_FRACTION;
    let transposition_table_size = cmp::max(1, total_size - pawn_hashtable_size);

    AllocationResult {
        transposition_table_size,
        pawn_hashtable_size,
    }
}
