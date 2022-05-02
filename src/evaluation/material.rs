use crate::state::board::Bitboard;
use crate::state::*;

/// Evaluates material on the `board` and returns score from the white color perspective (more than 0 when advantage, less than 0 when disadvantage).
/// This simple evaluator sums all scores of all present pieces using incremental counters in `board`, without considering the current game phase.
pub fn evaluate(board: &Bitboard) -> i16 {
    board.material_scores[WHITE as usize] - board.material_scores[BLACK as usize]
}

/// Recalculates incremental counters on the `board`. This function should be called only once during board initialization, as it's too slow in regular search.
pub fn recalculate_incremental_values(board: &mut Bitboard) {
    for color_index in 0..2 {
        let mut score = 0;
        for piece_index in 0..6 {
            score += (bit_count(board.pieces[color_index][piece_index]) as i16) * board.evaluation_parameters.piece_value[piece_index];
        }

        board.material_scores[color_index] = score;
    }
}
