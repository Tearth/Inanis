use super::*;
use crate::state::board::Bitboard;

/// Evaluates king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). Both additional parameters, `dangered_white_king_fields` and `dangered_black_king_fields`, are
/// calculated during mobility evaluation and are used here to get the final score.
pub fn evaluate(board: &Bitboard, dangered_white_king_fields: u32, dangered_black_king_fields: u32) -> i16 {
    evaluate_color(board, dangered_white_king_fields) - evaluate_color(board, dangered_black_king_fields)
}

/// Evaluates pawn structure on the `board` for the specified `color` and with `dangered_king_fields` count.
fn evaluate_color(board: &Bitboard, dangered_king_fields: u32) -> i16 {
    let game_phase = board.get_game_phase();
    let opening_score = (dangered_king_fields as i16) * board.evaluation_parameters.king_attacked_fields_opening;
    let ending_score = (dangered_king_fields as i16) * board.evaluation_parameters.king_attacked_fields_ending;

    taper_score(game_phase, opening_score, ending_score)
}
