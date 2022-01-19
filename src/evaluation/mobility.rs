use super::parameters::*;
use super::*;
use crate::state::board::*;
use crate::state::movescan::*;

pub fn evaluate(board: &Bitboard, dangered_white_king_fields: &mut u32, dangered_black_king_fields: &mut u32) -> i16 {
    evaluate_color(board, WHITE, dangered_black_king_fields) - evaluate_color(board, BLACK, dangered_white_king_fields)
}

fn evaluate_color(board: &Bitboard, color: u8, dangered_king_fields: &mut u32) -> i16 {
    let knight_mobility = get_piece_mobility::<KNIGHT>(board, color, dangered_king_fields);
    let bishop_mobility = get_piece_mobility::<BISHOP>(board, color, dangered_king_fields);
    let rook_mobility = get_piece_mobility::<ROOK>(board, color, dangered_king_fields);
    let queen_mobility = get_piece_mobility::<QUEEN>(board, color, dangered_king_fields);

    let game_phase = board.get_game_phase();
    let opening_score = (knight_mobility + bishop_mobility + rook_mobility + queen_mobility) * unsafe { MOBILITY_OPENING };
    let ending_score = (knight_mobility + bishop_mobility + rook_mobility + queen_mobility) * unsafe { MOBILITY_ENDING };

    taper_score(game_phase, opening_score, ending_score)
}
