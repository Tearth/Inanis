use super::*;
use crate::state::representation::Board;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoefficient;

/// Evaluates king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). Both additional parameters, `dangered_white_king_squares` and `dangered_black_king_squares`, are
/// calculated during mobility evaluation and are used here to get the final score.
pub fn evaluate(board: &Board, dangered_white_king_squares: u32, dangered_black_king_squares: u32) -> EvaluationResult {
    evaluate_color(board, dangered_white_king_squares) - evaluate_color(board, dangered_black_king_squares)
}

/// Evaluates pawn structure on the `board` for the specified `color` and with `dangered_king_squares` count.
fn evaluate_color(board: &Board, dangered_king_squares: u32) -> EvaluationResult {
    let index = (dangered_king_squares as usize).min(7);
    let opening_score = board.evaluation_parameters.king_attacked_squares_opening[index];
    let ending_score = board.evaluation_parameters.king_attacked_squares_ending[index];

    EvaluationResult::new(opening_score, ending_score)
}

#[cfg(feature = "dev")]
pub fn get_coefficients(dangered_white_king_squares: u32, dangered_black_king_squares: u32, index: &mut u16) -> Vec<TunerCoefficient> {
    let mut coefficients = Vec::new();

    for game_phase in ALL_PHASES {
        for i in 0..8 {
            let mut sum = 0;

            if dangered_white_king_squares == i || (i == 7 && dangered_white_king_squares > 7) {
                sum += 1;
            }
            if dangered_black_king_squares == i || (i == 7 && dangered_black_king_squares > 7) {
                sum -= 1;
            }

            if sum != 0 {
                coefficients.push(TunerCoefficient::new(sum, game_phase, *index));
            }

            *index += 1;
        }
    }

    coefficients
}
