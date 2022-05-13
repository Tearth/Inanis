use crate::state::movescan::Move;
use std::mem::MaybeUninit;

pub mod clock;
pub mod context;
pub mod history;
pub mod killers;
pub mod qsearch;
pub mod search;
pub mod see;

pub const MAX_DEPTH: i8 = 64;
pub const MIN_ALPHA: i16 = -CHECKMATE_SCORE;
pub const MIN_BETA: i16 = CHECKMATE_SCORE;
pub const DEADLINE_MULTIPLIER: f32 = 2.0;
pub const MAX_MOVES_COUNT: usize = 218;

pub const INVALID_SCORE: i16 = 0;
pub const DRAW_SCORE: i16 = 0;
pub const CHECKMATE_SCORE: i16 = 31900;

/// Checks if `score` is within mate range (from -[CHECKMATE_SCORE] to -[CHECKMATE_SCORE] + [MAX_DEPTH] and
/// from [CHECKMATE_SCORE] - [MAX_DEPTH] to [CHECKMATE_SCORE]).
pub fn is_score_near_checkmate(score: i16) -> bool {
    score.abs() >= CHECKMATE_SCORE - (MAX_DEPTH as i16) && score.abs() <= CHECKMATE_SCORE + (MAX_DEPTH as i16)
}

/// Performs a selection sort on `moves` and `move_scores` arrays with the length specified in `moves_count`, starting from `start_index`.
/// When it completes, the move and corresponding score will be under `start_index`, and the function returns move itself.
pub fn sort_next_move(moves: &mut [MaybeUninit<Move>], move_scores: &mut [MaybeUninit<i16>], start_index: usize, moves_count: usize) -> Move {
    let mut best_score = unsafe { move_scores[start_index].assume_init() };
    let mut best_index = start_index;

    for index in (start_index + 1)..moves_count {
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

    unsafe { moves[start_index].assume_init() }
}
