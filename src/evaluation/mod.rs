use crate::state::*;

pub mod material;
pub mod mobility;
pub mod parameters;
pub mod pawns;
pub mod pst;
pub mod safety;

pub static mut INITIAL_MATERIAL: i16 = 0;

pub fn init() {
    unsafe {
        INITIAL_MATERIAL = 0
            + 16 * parameters::PIECE_VALUE[PAWN as usize]
            + 4 * parameters::PIECE_VALUE[KNIGHT as usize]
            + 4 * parameters::PIECE_VALUE[BISHOP as usize]
            + 4 * parameters::PIECE_VALUE[ROOK as usize]
            + 2 * parameters::PIECE_VALUE[QUEEN as usize];
    }
}

pub fn taper_score(game_phase: f32, opening_score: i16, ending_score: i16) -> i16 {
    ((game_phase * (opening_score as f32)) + ((1.0 - game_phase) * (ending_score as f32))) as i16
}
