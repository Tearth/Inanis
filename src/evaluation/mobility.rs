use crate::state::board::*;
use crate::state::movescan::*;
use crate::state::*;

pub fn evaluate(board: &Bitboard, white_attack_mask: &mut u64, black_attack_mask: &mut u64) -> i16 {
    evaluate_color(board, WHITE, white_attack_mask) - evaluate_color(board, BLACK, black_attack_mask)
}

fn evaluate_color(board: &Bitboard, color: u8, attack_mask: &mut u64) -> i16 {
    let knight_mobility = get_piece_mobility::<KNIGHT>(board, color, attack_mask);
    let bishop_mobility = get_piece_mobility::<BISHOP>(board, color, attack_mask);
    let rook_mobility = get_piece_mobility::<ROOK>(board, color, attack_mask);
    let queen_mobility = get_piece_mobility::<QUEEN>(board, color, attack_mask);

    ((knight_mobility + bishop_mobility + rook_mobility + queen_mobility) * 2) as i16
}
