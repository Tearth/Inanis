use super::*;
use crate::state::representation::Board;
use crate::utils::bithelpers::BitHelpers;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoefficient;

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

/// Gets coefficients of piece-square table for `piece` on `board` and assigns indexes starting from `index`.
#[cfg(feature = "dev")]
pub fn get_coefficients(board: &Board, piece: usize, index: &mut u16) -> Vec<TunerCoefficient> {
    let mut coefficients = Vec::new();

    for game_phase in ALL_PHASES {
        for square in ALL_SQUARES {
            let current_index = 63 - square;
            let opposite_index = (square / 8) * 8 + (7 - (square % 8));

            let current_piece = board.piece_table[current_index];
            let opposite_piece = board.piece_table[opposite_index];

            let current_color = if (board.occupancy[WHITE] & (1 << current_index)) != 0 { WHITE } else { BLACK };
            let opposite_color = if (board.occupancy[WHITE] & (1 << opposite_index)) != 0 { WHITE } else { BLACK };

            if current_piece == piece as u8 && opposite_piece != piece as u8 && current_color == WHITE {
                coefficients.push(TunerCoefficient::new(1, game_phase, *index));
            } else if opposite_piece == piece as u8 && current_piece != piece as u8 && opposite_color == BLACK {
                coefficients.push(TunerCoefficient::new(-1, game_phase, *index));
            }

            *index += 1;
        }
    }

    coefficients
}
