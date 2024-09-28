use super::*;
use crate::{state::movescan::Move, utils::assert_fast};
use std::mem::MaybeUninit;

/// Performs a selection sort on `moves` and `move_scores` arrays with the length specified in `moves_count`, starting from `start_index`.
/// When it completes, the move and corresponding score will be under `start_index` - the function also explicitly returns both of them.
pub fn sort_next_move(
    moves: &mut [MaybeUninit<Move>; MAX_MOVES_COUNT],
    move_scores: &mut [MaybeUninit<i16>; MAX_MOVES_COUNT],
    start_index: usize,
    moves_count: usize,
) -> (Move, i16) {
    assert_fast!(start_index < MAX_MOVES_COUNT);
    assert_fast!(start_index <= moves_count);
    assert_fast!(start_index + moves_count < MAX_MOVES_COUNT);

    let mut best_score = unsafe { move_scores[start_index].assume_init() };
    let mut best_index = start_index;

    for index in (start_index + 1)..moves_count {
        assert_fast!(index < MAX_MOVES_COUNT);

        let score = unsafe { move_scores[index].assume_init() };
        if score > best_score {
            best_score = score;
            best_index = index;
        }
    }

    if best_index != start_index {
        moves.swap(start_index, best_index);
        move_scores.swap(start_index, best_index);
    }

    unsafe { (moves[start_index].assume_init(), best_score) }
}
