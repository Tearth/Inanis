use super::parameters::*;
use super::*;
use crate::state::board::*;
use crate::state::movescan::*;
use crate::state::*;

pub fn evaluate(board: &Bitboard, white_attack_mask: &mut u64, black_attack_mask: &mut u64) -> i16 {
    evaluate_color(board, WHITE, white_attack_mask) - evaluate_color(board, BLACK, black_attack_mask)
}

fn evaluate_color(board: &Bitboard, color: u8, attack_mask: &mut u64) -> i16 {
    let knight_mobility = get_piece_mobility::<KNIGHT>(board, color, attack_mask) as i16;
    let bishop_mobility = get_piece_mobility::<BISHOP>(board, color, attack_mask) as i16;
    let rook_mobility = get_piece_mobility::<ROOK>(board, color, attack_mask) as i16;
    let queen_mobility = get_piece_mobility::<QUEEN>(board, color, attack_mask) as i16;

    let game_phase = board.get_game_phase();
    let opening_score = (knight_mobility + bishop_mobility + rook_mobility + queen_mobility) * unsafe { MOBILITY_OPENING };
    let ending_score = (knight_mobility + bishop_mobility + rook_mobility + queen_mobility) * unsafe { MOBILITY_ENDING };

    taper_score(game_phase, opening_score, ending_score)
}
