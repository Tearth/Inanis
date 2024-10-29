use super::*;
use crate::state::representation::Board;
use mobility::EvalAux;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoeff;

/// Evaluates king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). Both additional parameters, `white_aux` and `black_aux`, are
/// calculated during mobility evaluation and are used here to get the final score.
pub fn evaluate(board: &Board, white_aux: &EvalAux, black_aux: &EvalAux) -> PackedEval {
    evaluate_color(board, white_aux) - evaluate_color(board, black_aux)
}

/// Evaluates king safety on the `board` for the specified `color` and `aux` data.
fn evaluate_color(_board: &Board, aux: &EvalAux) -> PackedEval {
    params::KING_AREA_THREATS[(aux.king_area_threats as usize).min(7)]
}

/// Gets coefficients of king safety for `board` and inserts them into `coeffs`. Similarly, their indices (starting from `index`) are inserted into `indices`.
/// Additionally, `white_aux` and `black_aux` calculated during mobility phase are also used here.
#[cfg(feature = "dev")]
pub fn get_coeffs(white_aux: &EvalAux, black_aux: &EvalAux, index: &mut u16, coeffs: &mut Vec<TunerCoeff>, indices: &mut Vec<u16>) {
    get_array_coeffs(white_aux.king_area_threats as u8, black_aux.king_area_threats as u8, 8, index, coeffs, indices)
}
