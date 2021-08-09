use super::values::*;
use crate::board::common::*;
use crate::board::repr::Bitboard;

pub fn evaluate(board: &Bitboard) -> i16 {
    let mut result = 0;

    for color_index in 0..=1 {
        for piece_index in 0..=5 {
            let mut pieces = board.pieces[color_index][piece_index];
            while pieces != 0 {
                pieces = pop_lsb(pieces);
                result += ((color_index as i16) * 2 - 1) * PIECE_VALUE[piece_index];
            }
        }
    }

    result
}
