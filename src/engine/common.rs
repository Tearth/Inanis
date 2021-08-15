use crate::state::movescan::Move;

pub const MAX_DEPTH: i8 = 32;
pub const CHECKMATE_SCORE: i16 = 31900;

pub fn is_score_near_checkmate(score: i16) -> bool {
    score.abs() >= CHECKMATE_SCORE - (MAX_DEPTH as i16) && score.abs() <= CHECKMATE_SCORE + (MAX_DEPTH as i16)
}

pub fn sort_next_move(moves: &mut [Move], move_scores: &mut [i16], start_index: usize, moves_count: usize) {
    let mut best_score = move_scores[start_index];
    let mut best_index = start_index;

    for index in (start_index + 1)..moves_count {
        if move_scores[index] > best_score {
            best_score = move_scores[index];
            best_index = index;
        }
    }

    if best_index != start_index {
        moves.swap(start_index, best_index);
        move_scores.swap(start_index, best_index);
    }
}
