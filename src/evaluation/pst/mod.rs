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

pub const KING_BUCKETS_COUNT: usize = 8;

#[rustfmt::skip]
pub const KING_BUCKETS: [usize; 64] = [
    7, 6, 5, 4, 3, 2, 1, 0,
    7, 6, 5, 4, 3, 2, 1, 0,
    7, 6, 5, 4, 3, 2, 1, 0,
    7, 6, 5, 4, 3, 2, 1, 0,
    7, 6, 5, 4, 3, 2, 1, 0,
    7, 6, 5, 4, 3, 2, 1, 0,
    7, 6, 5, 4, 3, 2, 1, 0,
    7, 6, 5, 4, 3, 2, 1, 0,
];

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
        let mut king_square = board.pieces[color_index][KING].bit_scan() & 0x3f;
        if color_index == BLACK {
            king_square = (1u64 << king_square).swap_bytes().bit_scan();
        }

        for phase in ALL_PHASES {
            let mut score = 0;
            for piece_index in ALL_PIECES {
                let mut pieces_bb = board.pieces[color_index][piece_index];
                while pieces_bb != 0 {
                    let square_bb = pieces_bb.get_lsb();
                    let mut square = square_bb.bit_scan();
                    pieces_bb = pieces_bb.pop_lsb();

                    if color_index == BLACK {
                        square = (1u64 << square).swap_bytes().bit_scan();
                    }

                    score += board.evaluation_parameters.get_pst_value(piece_index, king_square, phase, square);
                }
            }

            board.pst_scores[color_index][phase] = score;
        }
    }
}

/// Gets coefficients of piece-square table for `piece` on `board` and assigns indexes starting from `index`.
#[cfg(feature = "dev")]
pub fn get_coefficients(board: &Board, piece: usize, index: &mut u16, coefficients: &mut Vec<TunerCoefficient>, indices: &mut Vec<u16>) {
    for bucket in 0..KING_BUCKETS_COUNT {
        let valid_for_white = bucket == KING_BUCKETS[63 - board.pieces[WHITE][KING].bit_scan()];
        let valid_for_black = bucket == KING_BUCKETS[63 - board.pieces[BLACK][KING].bit_scan()];

        for game_phase in ALL_PHASES {
            for square in ALL_SQUARES {
                let current_index = 63 - square;
                let opposite_index = (square / 8) * 8 + (7 - (square % 8));

                let current_piece = board.piece_table[current_index];
                let opposite_piece = board.piece_table[opposite_index];

                let current_color = if (board.occupancy[WHITE] & (1 << current_index)) != 0 { WHITE } else { BLACK };
                let opposite_color = if (board.occupancy[WHITE] & (1 << opposite_index)) != 0 { WHITE } else { BLACK };

                if valid_for_white && !valid_for_black {
                    if current_piece == piece as u8 && current_color == WHITE {
                        indices.push(*index);
                        coefficients.push(TunerCoefficient::new(1, game_phase));
                    }
                } else if !valid_for_white && valid_for_black {
                    if opposite_piece == piece as u8 && opposite_color == BLACK {
                        indices.push(*index);
                        coefficients.push(TunerCoefficient::new(-1, game_phase));
                    }
                } else if valid_for_white && valid_for_black {
                    if current_piece == piece as u8 && opposite_piece != piece as u8 && current_color == WHITE {
                        indices.push(*index);
                        coefficients.push(TunerCoefficient::new(1, game_phase));
                    } else if opposite_piece == piece as u8 && current_piece != piece as u8 && opposite_color == BLACK {
                        indices.push(*index);
                        coefficients.push(TunerCoefficient::new(-1, game_phase));
                    }
                }

                *index += 1;
            }
        }
    }
}

#[cfg(feature = "dev")]
pub fn get_array_coefficients(
    white_feature: u8,
    black_feature: u8,
    max: u8,
    index: &mut u16,
    coefficients: &mut Vec<TunerCoefficient>,
    indices: &mut Vec<u16>,
) {
    for game_phase in ALL_PHASES {
        for i in 0..max {
            let mut sum = 0;

            if white_feature == i || (i == max - 1 && white_feature > max - 1) {
                sum += 1;
            }
            if black_feature == i || (i == max - 1 && black_feature > max - 1) {
                sum -= 1;
            }

            if sum != 0 {
                indices.push(*index);
                coefficients.push(TunerCoefficient::new(sum, game_phase));
            }

            *index += 1;
        }
    }
}
