use super::parameters;
use super::*;
use crate::state::board::Bitboard;

pub fn evaluate(board: &Bitboard, dangered_white_king_fields: u32, dangered_black_king_fields: u32) -> i16 {
    evaluate_color(board, dangered_white_king_fields) - evaluate_color(board, dangered_black_king_fields)
}

fn evaluate_color(board: &Bitboard, dangered_king_fields: u32) -> i16 {
    let game_phase = board.get_game_phase();
    let opening_score = (dangered_king_fields as i16) * unsafe { parameters::KING_ATTACKED_FIELDS_OPENING };
    let ending_score = (dangered_king_fields as i16) * unsafe { parameters::KING_ATTACKED_FIELDS_ENDING };

    taper_score(game_phase, opening_score, ending_score)
}
