use super::*;
use crate::state::board::Bitboard;

/// Evaluates king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). Both additional parameters, `dangered_white_king_squares` and `dangered_black_king_squares`, are
/// calculated during mobility evaluation and are used here to get the final score.
pub fn evaluate(board: &Bitboard, dangered_white_king_squares: u32, dangered_black_king_squares: u32) -> i16 {
    evaluate_color(board, dangered_white_king_squares) - evaluate_color(board, dangered_black_king_squares)
}

/// Evaluates pawn structure on the `board` for the specified `color` and with `dangered_king_squares` count.
fn evaluate_color(board: &Bitboard, dangered_king_squares: u32) -> i16 {
    let game_phase = board.get_game_phase();
    let opening_score = (dangered_king_squares as i16).pow(2) * board.evaluation_parameters.king_attacked_squares_opening;
    let ending_score = (dangered_king_squares as i16).pow(2) * board.evaluation_parameters.king_attacked_squares_ending;

    taper_score(game_phase, opening_score, ending_score)
}
