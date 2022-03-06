use crate::state::*;

pub mod material;
pub mod mobility;
pub mod parameters;
pub mod pawns;
pub mod pst;
pub mod safety;

pub static mut INITIAL_MATERIAL: i16 = 0;

/// Initializes dynamic evaluation parameters (like [INITIAL_MATERIAL]).
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

/// Blends `opening_score` and `ending_score` with the ratio passed in `game_phase`. The ratio is a number from 0.0 to 1.0, where:
///  - 1.0 represents a board with the initial state set (opening phase)
///  - 0.0 represents a board without any piece (ending phase)
///  - every value between them represents a board state somewhere in the middle game
pub fn taper_score(game_phase: f32, opening_score: i16, ending_score: i16) -> i16 {
    ((game_phase * (opening_score as f32)) + ((1.0 - game_phase) * (ending_score as f32))) as i16
}
