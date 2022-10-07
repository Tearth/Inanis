use std::cmp;

const PAWN_HASHTABLE_SIZE_FRACTION: usize = 64;

pub struct AllocationResult {
    pub transposition_table_size: usize,
    pub pawn_hashtable_size: usize,
}

impl AllocationResult {
    /// Constructs a new instance of [AllocationResult] with stored `id`, `board` and `best_move`.
    pub fn new(transposition_table_size: usize, pawn_hashtable_size: usize) -> Self {
        Self { transposition_table_size, pawn_hashtable_size }
    }
}

/// Calculates optimal size for the hashtables based on `total_size` megabytes of available memory. For every [PAWN_HASHTABLE_SIZE_FRACTION] megabytes
/// of transposition table, there will be 1 MB of pawn hashtable. Minimal size for each of them is 1 MB.
pub fn get_allocation(total_size: usize) -> AllocationResult {
    let pawn_hashtable_size = 1 + total_size / PAWN_HASHTABLE_SIZE_FRACTION;
    let transposition_table_size = cmp::max(1, total_size);

    AllocationResult::new(transposition_table_size, pawn_hashtable_size)
}
