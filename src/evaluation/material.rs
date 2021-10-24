use crate::state::board::Bitboard;
use crate::state::*;

static PIECE_VALUE: [i16; 6] = [100, 300, 330, 500, 1100, 10000];

pub fn evaluate(board: &Bitboard) -> i16 {
    board.material_scores[WHITE as usize] - board.material_scores[BLACK as usize]
}

pub fn get_value(piece: u8) -> i16 {
    PIECE_VALUE[piece as usize]
}

pub fn recalculate_incremental_values(board: &mut Bitboard) {
    for color_index in 0..=1 {
        let mut score = 0;
        for piece_index in 0..=5 {
            let mut pieces = board.pieces[color_index][piece_index];
            while pieces != 0 {
                pieces = pop_lsb(pieces);
                score += PIECE_VALUE[piece_index];
            }
        }

        board.material_scores[color_index] = score;
    }
}
