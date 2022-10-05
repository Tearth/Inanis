use super::*;
use crate::{state::representation::Board, utils::bithelpers::BitHelpers};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

/// Evaluates piece-square table value on the `board` and returns score from the white color perspective (more than 0 when advantage, less than 0 when disadvantage).
/// This evaluator sums all values of the pieces for the specified squares, using incremental counters in `board`.
pub fn evaluate(board: &Board) -> EvaluationResult {
    let opening_score = board.pst_scores[WHITE as usize][OPENING as usize] - board.pst_scores[BLACK as usize][OPENING as usize];
    let ending_score = board.pst_scores[WHITE as usize][ENDING as usize] - board.pst_scores[BLACK as usize][ENDING as usize];

    EvaluationResult::new(opening_score, ending_score)
}

/// Recalculates incremental counters on the `board`. This function should be called only once during board initialization, as it's too slow in regular search.
pub fn recalculate_incremental_values(board: &mut Board) {
    for color_index in 0..2 {
        for phase in 0..2 {
            let mut score = 0;
            for piece_index in 0..6 {
                let mut pieces = board.pieces[color_index][piece_index];
                while pieces != 0 {
                    let square = pieces.get_lsb();
                    let square_index = square.bit_scan();
                    pieces = pieces.pop_lsb();

                    score += board.evaluation_parameters.pst[color_index as usize][piece_index as usize][phase as usize][square_index as usize] as i16;
                }
            }

            board.pst_scores[color_index][phase as usize] = score;
        }
    }
}
