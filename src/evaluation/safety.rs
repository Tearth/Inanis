use super::parameters::*;
use super::*;
use crate::state::board::Bitboard;
use crate::state::patterns::*;

pub fn evaluate(board: &Bitboard, white_attack_mask: u64, black_attack_mask: u64) -> i16 {
    evaluate_color(board, WHITE, black_attack_mask) - evaluate_color(board, BLACK, white_attack_mask)
}

fn evaluate_color(board: &Bitboard, color: u8, enemy_attack_mask: u64) -> i16 {
    let king = board.pieces[color as usize][KING as usize];
    let king_field = bit_scan(king);
    let attacked_fields = bit_count(get_box(king_field as usize) & enemy_attack_mask);

    let game_phase = board.get_game_phase();
    let opening_score = (attacked_fields as i16) * unsafe { KING_ATTACKED_FIELDS_OPENING };
    let ending_score = (attacked_fields as i16) * unsafe { KING_ATTACKED_FIELDS_ENDING };

    taper_score(game_phase, opening_score, ending_score)
}
