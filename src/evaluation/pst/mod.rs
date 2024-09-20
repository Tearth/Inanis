use super::*;
use crate::evaluation;
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

pub use bishop::BISHOP_PST_PATTERN;
pub use king::KING_PST_PATTERN;
pub use knight::KNIGHT_PST_PATTERN;
pub use pawn::PAWN_PST_PATTERN;
pub use queen::QUEEN_PST_PATTERN;
pub use rook::ROOK_PST_PATTERN;

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
pub fn evaluate(board: &Board) -> PackedEval {
    board.state.pst_score
}

/// Recalculates incremental counters on the `board`. This function should be called only if really necessary, as it's too slow in regular search.
pub fn recalculate_incremental_values(board: &mut Board) {
    let mut score = PackedEval::default();

    for color in ALL_COLORS {
        let sign = -(color as i16 * 2 - 1);
        let king_bb = board.pieces[color][KING];
        let king_square = match color == WHITE {
            true => king_bb.bit_scan() % 64,
            false => king_bb.swap_bytes().bit_scan() % 64,
        };

        for piece_index in ALL_PIECES {
            let mut pieces_bb = board.pieces[color][piece_index];
            while pieces_bb != 0 {
                let square_bb = pieces_bb.get_lsb();
                let mut square = square_bb.bit_scan();
                pieces_bb = pieces_bb.pop_lsb();

                if color == BLACK {
                    square = (1u64 << square).swap_bytes().bit_scan();
                }

                score += sign * evaluation::get_pst_value(piece_index, king_square, square);
            }
        }
    }

    board.state.pst_score = score;
}

/// Gets a PST value for the specified `color`, `piece`, `phase` and `square`.
pub fn get_pst_value(piece: usize, king_square: usize, square: usize) -> PackedEval {
    debug_assert!(piece <= 6);
    debug_assert!(king_square < 64);
    debug_assert!(square < 64);

    let pst = match piece {
        PAWN => &pst::PAWN_PST_PATTERN,
        KNIGHT => &pst::KNIGHT_PST_PATTERN,
        BISHOP => &pst::BISHOP_PST_PATTERN,
        ROOK => &pst::ROOK_PST_PATTERN,
        QUEEN => &pst::QUEEN_PST_PATTERN,
        KING => &pst::KING_PST_PATTERN,
        _ => panic_fast!("Invalid value: piece={}", piece),
    };

    pst[KING_BUCKETS[63 - king_square]][63 - square]
}

/// Gets coefficients of piece-square table for `piece` on `board` and inserts them into `coefficients`.
/// Similarly, their indices (starting from `index`) are inserted into `indices`.
#[cfg(feature = "dev")]
pub fn get_coefficients(board: &Board, piece: usize, index: &mut u16, coefficients: &mut Vec<TunerCoefficient>, indices: &mut Vec<u16>) {
    debug_assert!(piece <= 6);

    for bucket in 0..KING_BUCKETS_COUNT {
        let valid_for_white = bucket == KING_BUCKETS[63 - board.pieces[WHITE][KING].bit_scan()];
        let valid_for_black = bucket == KING_BUCKETS[63 - board.pieces[BLACK][KING].bit_scan()];

        for square in ALL_SQUARES {
            let current_index = 63 - square;
            let opposite_index = (1u64 << current_index).swap_bytes().bit_scan();

            let current_piece = board.piece_table[current_index];
            let opposite_piece = board.piece_table[opposite_index];

            let current_color = if (board.occupancy[WHITE] & (1 << current_index)) != 0 { WHITE } else { BLACK };
            let opposite_color = if (board.occupancy[WHITE] & (1 << opposite_index)) != 0 { WHITE } else { BLACK };

            if valid_for_white && !valid_for_black {
                if current_piece == piece as u8 && current_color == WHITE {
                    coefficients.push(TunerCoefficient::new(1, OPENING));
                    coefficients.push(TunerCoefficient::new(1, ENDING));
                    indices.push(*index);
                    indices.push(*index + 1);
                }
            } else if !valid_for_white && valid_for_black {
                if opposite_piece == piece as u8 && opposite_color == BLACK {
                    coefficients.push(TunerCoefficient::new(-1, OPENING));
                    coefficients.push(TunerCoefficient::new(-1, ENDING));
                    indices.push(*index);
                    indices.push(*index + 1);
                }
            } else if valid_for_white && valid_for_black {
                if current_piece == piece as u8 && opposite_piece != piece as u8 && current_color == WHITE {
                    coefficients.push(TunerCoefficient::new(1, OPENING));
                    coefficients.push(TunerCoefficient::new(1, ENDING));
                    indices.push(*index);
                    indices.push(*index + 1);
                } else if opposite_piece == piece as u8 && current_piece != piece as u8 && opposite_color == BLACK {
                    coefficients.push(TunerCoefficient::new(-1, OPENING));
                    coefficients.push(TunerCoefficient::new(-1, ENDING));
                    indices.push(*index);
                    indices.push(*index + 1);
                }
            }

            *index += 2;
        }
    }
}

/// Gets coefficients for a specific feature (`white_data`/`black_data`/`max`) and inserts them into `coefficients`.
/// Similarly, their indices (starting from `index`) are inserted into `indices`.
#[cfg(feature = "dev")]
pub fn get_array_coefficients(white_data: u8, black_data: u8, max: u8, index: &mut u16, coefficients: &mut Vec<TunerCoefficient>, indices: &mut Vec<u16>) {
    use std::cmp;

    let white_data = cmp::min(white_data, max - 1);
    let black_data = cmp::min(black_data, max - 1);

    for i in 0..max {
        let sum = (white_data == i) as i8 - (black_data == i) as i8;
        if sum != 0 {
            coefficients.push(TunerCoefficient::new(sum, OPENING));
            coefficients.push(TunerCoefficient::new(sum, ENDING));
            indices.push(*index);
            indices.push(*index + 1);
        }

        *index += 2;
    }
}
