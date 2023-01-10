use super::*;
use crate::state::representation::Board;
use crate::utils::bithelpers::BitHelpers;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

/// Evaluates piece-square table value on the `board` and returns score from the white color perspective (more than 0 when advantage, less than 0 when disadvantage).
/// This evaluator sums all values of the pieces for the specified squares, using incremental counters in `board`.
pub fn evaluate(board: &Board) -> EvaluationResult {
    let opening_score = board.pst_scores[WHITE][OPENING] - board.pst_scores[BLACK][OPENING];
    let ending_score = board.pst_scores[WHITE][ENDING] - board.pst_scores[BLACK][ENDING];

    EvaluationResult::new(opening_score, ending_score)
}

/// Recalculates incremental counters on the `board`. This function should be called only once during board initialization, as it's too slow in regular search.
pub fn recalculate_incremental_values(board: &mut Board) {
    for color_index in ALL_COLORS {
        for phase in ALL_PHASES {
            let mut score = 0;
            for piece_index in ALL_PIECES {
                let mut pieces_bb = board.pieces[color_index][piece_index];
                while pieces_bb != 0 {
                    let square_bb = pieces_bb.get_lsb();
                    let square = square_bb.bit_scan();
                    pieces_bb = pieces_bb.pop_lsb();

                    score += board.evaluation_parameters.pst[color_index][piece_index][phase][square];
                }
            }

            board.pst_scores[color_index][phase] = score;
        }
    }
}
