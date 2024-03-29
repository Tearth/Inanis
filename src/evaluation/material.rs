use crate::state::representation::Board;
use crate::state::*;
use crate::utils::bithelpers::BitHelpers;

/// Evaluates material on the `board` and returns score from the white color perspective (more than 0 when advantage, less than 0 when disadvantage).
/// This simple evaluator sums all scores of all present pieces using incremental counters in `board`, without considering the current game phase.
pub fn evaluate(board: &Board) -> i16 {
    board.material_scores[WHITE] - board.material_scores[BLACK]
}

/// Recalculates incremental counters on the `board`. This function should be called only once during board initialization, as it's too slow in regular search.
pub fn recalculate_incremental_values(board: &mut Board) {
    for color_index in ALL_COLORS {
        let mut score = 0;
        for piece_index in ALL_PIECES {
            let pieces_count = board.pieces[color_index][piece_index].bit_count();
            score += (pieces_count as i16) * board.evaluation_parameters.piece_value[piece_index];
        }

        board.material_scores[color_index] = score;
    }
}
