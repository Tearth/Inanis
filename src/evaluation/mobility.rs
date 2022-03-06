use super::*;
use crate::state::board::Bitboard;
use crate::state::movescan;

/// Evaluates mobility and part of the king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). This evaluator does two things at once: first, counts all possible moves of knight, bishop, rook, queen
/// (pawns and king are too slow and not very important), and second, sums how many fields around both kings are dangered by enemy side
/// (`dangered_white_king_fields` and `dangered_black_king_fields`). This is used in the safety evaluator, to prevent calculating the same thing twice.
pub fn evaluate(board: &Bitboard, dangered_white_king_fields: &mut u32, dangered_black_king_fields: &mut u32) -> i16 {
    evaluate_color(board, WHITE, dangered_black_king_fields) - evaluate_color(board, BLACK, dangered_white_king_fields)
}

/// Evaluates mobility and `dangered_king_fields` on the `board` for the specified `color`.
fn evaluate_color(board: &Bitboard, color: u8, dangered_king_fields: &mut u32) -> i16 {
    let knight_mobility = movescan::get_piece_mobility::<KNIGHT>(board, color, dangered_king_fields);
    let bishop_mobility = movescan::get_piece_mobility::<BISHOP>(board, color, dangered_king_fields);
    let rook_mobility = movescan::get_piece_mobility::<ROOK>(board, color, dangered_king_fields);
    let queen_mobility = movescan::get_piece_mobility::<QUEEN>(board, color, dangered_king_fields);

    let game_phase = board.get_game_phase();
    let opening_score = (knight_mobility + bishop_mobility + rook_mobility + queen_mobility) * unsafe { parameters::MOBILITY_OPENING };
    let ending_score = (knight_mobility + bishop_mobility + rook_mobility + queen_mobility) * unsafe { parameters::MOBILITY_ENDING };

    taper_score(game_phase, opening_score, ending_score)
}
