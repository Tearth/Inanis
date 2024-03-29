use super::*;
use crate::state::movescan;
use crate::state::representation::Board;

/// Evaluates mobility and part of the king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). This evaluator does two things at once: first, counts all possible moves of knight, bishop, rook, queen
/// (pawns and king are too slow and not very important), and second, sums how many squares around both kings are dangered by enemy side
/// (`dangered_white_king_squares` and `dangered_black_king_squares`). This is used in the safety evaluator, to prevent calculating the same thing twice.
pub fn evaluate(board: &Board, dangered_white_king_squares: &mut u32, dangered_black_king_squares: &mut u32) -> EvaluationResult {
    evaluate_color(board, WHITE, dangered_black_king_squares) - evaluate_color(board, BLACK, dangered_white_king_squares)
}

/// Evaluates mobility and `dangered_king_squares` on the `board` for the specified `color`.
fn evaluate_color(board: &Board, color: usize, dangered_king_squares: &mut u32) -> EvaluationResult {
    let knight_mobility = movescan::get_piece_mobility::<KNIGHT>(board, color, dangered_king_squares);
    let bishop_mobility = movescan::get_piece_mobility::<BISHOP>(board, color, dangered_king_squares);
    let rook_mobility = movescan::get_piece_mobility::<ROOK>(board, color, dangered_king_squares);
    let queen_mobility = movescan::get_piece_mobility::<QUEEN>(board, color, dangered_king_squares);

    let knight_mobility_opening_score = knight_mobility * board.evaluation_parameters.mobility_opening[KNIGHT];
    let knight_mobility_ending_score = knight_mobility * board.evaluation_parameters.mobility_ending[KNIGHT];

    let bishop_mobility_opening_score = bishop_mobility * board.evaluation_parameters.mobility_opening[BISHOP];
    let bishop_mobility_ending_score = bishop_mobility * board.evaluation_parameters.mobility_ending[BISHOP];

    let rook_mobility_opening_score = rook_mobility * board.evaluation_parameters.mobility_opening[ROOK];
    let rook_mobility_ending_score = rook_mobility * board.evaluation_parameters.mobility_ending[ROOK];

    let queen_mobility_opening_score = queen_mobility * board.evaluation_parameters.mobility_opening[QUEEN];
    let queen_mobility_ending_score = queen_mobility * board.evaluation_parameters.mobility_ending[QUEEN];

    let opening_score = knight_mobility_opening_score + bishop_mobility_opening_score + rook_mobility_opening_score + queen_mobility_opening_score;
    let ending_score = knight_mobility_ending_score + bishop_mobility_ending_score + rook_mobility_ending_score + queen_mobility_ending_score;

    EvaluationResult::new(opening_score, ending_score)
}
