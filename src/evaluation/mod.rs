use crate::state::*;
use crate::utils::panic_unchecked;
use pst::*;
use std::ops;

pub mod material;
pub mod mobility;
pub mod parameters;
pub mod pawns;
pub mod pst;
pub mod safety;

pub const INITIAL_GAME_PHASE: u8 = 24;
pub const PIECE_PHASE_VALUE: [u8; 6] = [0, 1, 1, 2, 4, 0];

#[derive(Clone)]
pub struct EvaluationParameters {
    pub piece_value: [i16; 6],

    pub bishop_pair_opening: i16,
    pub bishop_pair_ending: i16,

    pub mobility_inner_opening: [i16; 6],
    pub mobility_inner_ending: [i16; 6],

    pub mobility_outer_opening: [i16; 6],
    pub mobility_outer_ending: [i16; 6],

    pub doubled_pawn_opening: [i16; 8],
    pub doubled_pawn_ending: [i16; 8],

    pub isolated_pawn_opening: [i16; 8],
    pub isolated_pawn_ending: [i16; 8],

    pub chained_pawn_opening: [i16; 8],
    pub chained_pawn_ending: [i16; 8],

    pub passed_pawn_opening: [i16; 8],
    pub passed_pawn_ending: [i16; 8],

    pub pawn_shield_opening: [i16; 8],
    pub pawn_shield_ending: [i16; 8],

    pub pawn_shield_open_file_opening: [i16; 8],
    pub pawn_shield_open_file_ending: [i16; 8],

    pub king_attacked_squares_opening: [i16; 8],
    pub king_attacked_squares_ending: [i16; 8],
}

pub struct EvaluationResult {
    pub opening_score: i16,
    pub ending_score: i16,
}

impl EvaluationParameters {
    /// Gets a PST value for the specified `color`, `piece`, `phase` and `square`.
    pub fn get_pst_value(&self, piece: usize, king_square: usize, phase: usize, square: usize) -> i16 {
        let pst = match piece {
            PAWN => &Self::PAWN_PST_PATTERN,
            KNIGHT => &Self::KNIGHT_PST_PATTERN,
            BISHOP => &Self::BISHOP_PST_PATTERN,
            ROOK => &Self::ROOK_PST_PATTERN,
            QUEEN => &Self::QUEEN_PST_PATTERN,
            KING => &Self::KING_PST_PATTERN,
            _ => panic_unchecked!("Invalid value: piece={}", piece),
        };

        pst[KING_BUCKETS[63 - king_square]][phase][63 - square]
    }
}

impl EvaluationResult {
    /// Constructs a new instance of [EvaluationResult] with stored `opening_score` and `ending_score`.
    pub fn new(opening_score: i16, ending_score: i16) -> EvaluationResult {
        EvaluationResult { opening_score, ending_score }
    }

    /// Blends `opening_score` and `ending_score` with the ratio passed in `game_phase`. The ratio is a number from 0 to `max_game_phase`, where:
    ///  - `max_game_phase` represents a board with the initial state set (opening phase)
    ///  - 0 represents a board without any piece (ending phase)
    ///  - every value between them represents a board state somewhere in the middle game
    pub fn taper_score(&self, game_phase: u8) -> i16 {
        let opening_score = (self.opening_score as i32) * (game_phase as i32);
        let ending_score = (self.ending_score as i32) * ((INITIAL_GAME_PHASE as i32) - (game_phase as i32));

        ((opening_score + ending_score) / (INITIAL_GAME_PHASE as i32)) as i16
    }
}

impl ops::Add<i16> for EvaluationResult {
    type Output = EvaluationResult;

    fn add(self, rhs: i16) -> EvaluationResult {
        EvaluationResult::new(self.opening_score + rhs, self.ending_score + rhs)
    }
}

impl ops::Add<EvaluationResult> for i16 {
    type Output = EvaluationResult;

    fn add(self, rhs: EvaluationResult) -> EvaluationResult {
        EvaluationResult::new(self + rhs.opening_score, self + rhs.ending_score)
    }
}

impl ops::Add<EvaluationResult> for EvaluationResult {
    type Output = EvaluationResult;

    fn add(self, rhs: EvaluationResult) -> EvaluationResult {
        EvaluationResult::new(self.opening_score + rhs.opening_score, self.ending_score + rhs.ending_score)
    }
}

impl ops::Sub<EvaluationResult> for EvaluationResult {
    type Output = EvaluationResult;

    fn sub(self, rhs: EvaluationResult) -> EvaluationResult {
        EvaluationResult::new(self.opening_score - rhs.opening_score, self.ending_score - rhs.ending_score)
    }
}
