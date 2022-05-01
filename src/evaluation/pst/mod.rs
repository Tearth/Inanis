use super::*;
use crate::state::board::Bitboard;

#[rustfmt::skip]
pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

pub fn evaluate(board: &Bitboard) -> i16 {
    let game_phase = board.get_game_phase();
    let opening_score = board.pst_scores[WHITE as usize][OPENING as usize] - board.pst_scores[BLACK as usize][OPENING as usize];
    let ending_score = board.pst_scores[WHITE as usize][ENDING as usize] - board.pst_scores[BLACK as usize][ENDING as usize];

    taper_score(game_phase, opening_score, ending_score)
}

pub fn recalculate_incremental_values(board: &mut Bitboard) {
    for color_index in 0..2 {
        for phase in [OPENING, ENDING] {
            let mut score = 0;
            for piece_index in 0..6 {
                let mut pieces = board.pieces[color_index][piece_index];
                while pieces != 0 {
                    let field = get_lsb(pieces);
                    let field_index = bit_scan(field);
                    pieces = pop_lsb(pieces);

                    score +=
                        unsafe { board.evaluation_parameters.pst[piece_index as usize][color_index as usize][phase as usize][field_index as usize] as i16 };
                }
            }

            board.pst_scores[color_index][phase as usize] = score;
        }
    }
}
