use super::PackedEval;
use crate::evaluation::params;
use crate::state::representation::Board;
use crate::state::*;
use crate::utils::bithelpers::BitHelpers;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoeff;

/// Evaluates material on the `board` and returns score from the white color perspective (more than 0 when advantage, less than 0 when disadvantage).
/// This simple evaluator sums all scores of all present pieces using incremental counters in `board`, without considering the current game phase.
pub fn evaluate(board: &Board) -> PackedEval {
    let white_has_bishop_pair = if board.pieces[WHITE][BISHOP].bit_count() == 2 { 1 } else { 0 };
    let black_has_bishop_pair = if board.pieces[BLACK][BISHOP].bit_count() == 2 { 1 } else { 0 };

    (white_has_bishop_pair - black_has_bishop_pair) * params::BISHOP_PAIR
}

/// Gets coefficients of material for `board` and inserts them into `coefficients`. Similarly, their indices (starting from `index`) are inserted into `indices`.
#[cfg(feature = "dev")]
pub fn get_coeffs(board: &Board, index: &mut u16, coeffs: &mut Vec<TunerCoeff>, indices: &mut Vec<u16>) {
    let mut data = [
        TunerCoeff::new(board.pieces[WHITE][PAWN].bit_count() as i8 - board.pieces[BLACK][PAWN].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][KNIGHT].bit_count() as i8 - board.pieces[BLACK][KNIGHT].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][BISHOP].bit_count() as i8 - board.pieces[BLACK][BISHOP].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][ROOK].bit_count() as i8 - board.pieces[BLACK][ROOK].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][QUEEN].bit_count() as i8 - board.pieces[BLACK][QUEEN].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][KING].bit_count() as i8 - board.pieces[BLACK][KING].bit_count() as i8, OPENING),
    ];

    for coeff in &mut data {
        let (value, _) = coeff.get_data();
        if value != 0 {
            coeffs.push(coeff.clone());
            indices.push(*index);
        }

        *index += 1;
    }

    let white_has_bishop_pair = if board.pieces[WHITE][BISHOP].bit_count() == 2 { 1 } else { 0 };
    let black_has_bishop_pair = if board.pieces[BLACK][BISHOP].bit_count() == 2 { 1 } else { 0 };
    let bishop_pair_diff = white_has_bishop_pair - black_has_bishop_pair;

    if bishop_pair_diff != 0 {
        coeffs.push(TunerCoeff::new(bishop_pair_diff, OPENING));
        coeffs.push(TunerCoeff::new(bishop_pair_diff, ENDING));
        indices.push(*index);
        indices.push(*index + 1);
    }

    *index += 2;
}
