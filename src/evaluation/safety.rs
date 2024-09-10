use super::*;
use crate::state::representation::Board;
use mobility::MobilityAuxData;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoefficient;

/// Evaluates king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). Both additional parameters, `dangered_white_king_squares` and `dangered_black_king_squares`, are
/// calculated during mobility evaluation and are used here to get the final score.
pub fn evaluate(board: &Board, white_aux: &MobilityAuxData, black_aux: &MobilityAuxData) -> PackedEval {
    evaluate_color(board, white_aux) - evaluate_color(board, black_aux)
}

/// Evaluates king safety on the `board` for the specified `color` and with `dangered_king_squares` count.
fn evaluate_color(_board: &Board, aux: &MobilityAuxData) -> PackedEval {
    let index = (aux.king_area_threats as usize).min(7);
    params::KING_AREA_THREATS[index]
}

#[cfg(feature = "dev")]
pub fn get_coefficients(
    white_aux: &MobilityAuxData,
    black_aux: &MobilityAuxData,
    index: &mut u16,
    coefficients: &mut Vec<TunerCoefficient>,
    indices: &mut Vec<u16>,
) {
    get_array_coefficients(white_aux.king_area_threats as u8, black_aux.king_area_threats as u8, 8, index, coefficients, indices)
}
