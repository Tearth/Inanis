use super::*;
use crate::state::representation::Board;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoefficient;

/// Evaluates king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). Both additional parameters, `dangered_white_king_squares` and `dangered_black_king_squares`, are
/// calculated during mobility evaluation and are used here to get the final score.
pub fn evaluate(board: &Board, dangered_white_king_squares: u32, dangered_black_king_squares: u32) -> PackedEval {
    evaluate_color(board, dangered_white_king_squares) - evaluate_color(board, dangered_black_king_squares)
}

/// Evaluates king safety on the `board` for the specified `color` and with `dangered_king_squares` count.
fn evaluate_color(_board: &Board, dangered_king_squares: u32) -> PackedEval {
    let index = (dangered_king_squares as usize).min(7);
    params::KING_ATTACKED_SQUARES[index]
}

#[cfg(feature = "dev")]
pub fn get_coefficients(
    dangered_white_king_squares: u32,
    dangered_black_king_squares: u32,
    index: &mut u16,
    coefficients: &mut Vec<TunerCoefficient>,
    indices: &mut Vec<u16>,
) {
    get_array_coefficients(dangered_white_king_squares as u8, dangered_black_king_squares as u8, 8, index, coefficients, indices)
}
