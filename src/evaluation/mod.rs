use crate::state::*;
use std::ops;

pub mod material;
pub mod mobility;
pub mod parameters;
pub mod pawns;
pub mod pst;
pub mod safety;

#[derive(Clone)]
pub struct EvaluationParameters {
    pub piece_value: [i16; 6],
    pub piece_phase_value: [u8; 6],
    pub initial_game_phase: u8,

    pub mobility_opening: [i16; 6],
    pub mobility_ending: [i16; 6],
    pub mobility_center_multiplier: [i16; 6],

    pub doubled_pawn_opening: i16,
    pub doubled_pawn_ending: i16,

    pub isolated_pawn_opening: i16,
    pub isolated_pawn_ending: i16,

    pub chained_pawn_opening: i16,
    pub chained_pawn_ending: i16,

    pub passed_pawn_opening: i16,
    pub passed_pawn_ending: i16,

    pub pawn_shield_opening: i16,
    pub pawn_shield_ending: i16,

    pub pawn_shield_open_file_opening: i16,
    pub pawn_shield_open_file_ending: i16,

    pub king_attacked_squares_opening: i16,
    pub king_attacked_squares_ending: i16,

    pub pst: [[[[i16; 64]; 2]; 6]; 2],
    pub pst_patterns: [[[i16; 64]; 2]; 6],
}

pub struct EvaluationResult {
    pub opening_score: i16,
    pub ending_score: i16,
}

impl EvaluationParameters {
    /// Initializes PST patterns with used by default during search.
    fn set_default_pst_patterns(&mut self) {
        self.pst_patterns[PAWN as usize] = self.get_pawn_pst_pattern();
        self.pst_patterns[KNIGHT as usize] = self.get_knight_pst_pattern();
        self.pst_patterns[BISHOP as usize] = self.get_bishop_pst_pattern();
        self.pst_patterns[ROOK as usize] = self.get_rook_pst_pattern();
        self.pst_patterns[QUEEN as usize] = self.get_queen_pst_pattern();
        self.pst_patterns[KING as usize] = self.get_king_pst_pattern();
    }

    /// Recalculates initial material and PST tables.
    pub fn recalculate(&mut self) {
        for color in WHITE..=BLACK {
            for piece in PAWN..=KING {
                for phase in OPENING..=ENDING {
                    self.pst[color as usize][piece as usize][phase as usize] = self.calculate_pst(color, &self.pst_patterns[piece as usize][phase as usize]);
                }
            }
        }
    }

    /// Calculates PST table for the specified `color` and `pattern`.
    fn calculate_pst(&self, color: u8, pattern: &[i16; 64]) -> [i16; 64] {
        let mut array = [0; 64];

        match color {
            WHITE => {
                for square_index in A1..=H8 {
                    array[square_index as usize] = pattern[(63 - square_index) as usize];
                }
            }
            BLACK => {
                for file in FILE_A..=FILE_H {
                    for rank in RANK_1..=RANK_8 {
                        array[(file + rank * 8) as usize] = pattern[((7 - file) + rank * 8) as usize];
                    }
                }
            }
            _ => panic!("Invalid parameter: color={}", color),
        }

        array
    }

    /// Gets a PST value for the specified `color`, `piece`, `phase` and `square`.
    pub fn get_pst_value(&self, color: u8, piece: u8, phase: u8, square: u8) -> i16 {
        self.pst[color as usize][piece as usize][phase as usize][square as usize] as i16
    }
}

impl EvaluationResult {
    pub fn new(opening_score: i16, ending_score: i16) -> EvaluationResult {
        EvaluationResult { opening_score, ending_score }
    }

    /// Blends `opening_score` and `ending_score` with the ratio passed in `game_phase`. The ratio is a number from 0 to `max_game_phase`, where:
    ///  - `max_game_phase` represents a board with the initial state set (opening phase)
    ///  - 0 represents a board without any piece (ending phase)
    ///  - every value between them represents a board state somewhere in the middle game
    pub fn taper_score(&self, game_phase: u8, max_game_phase: u8) -> i16 {
        let opening_score = (self.opening_score as i32) * (game_phase as i32);
        let ending_score = (self.ending_score as i32) * ((max_game_phase as i32) - (game_phase as i32));

        ((opening_score + ending_score) / (max_game_phase as i32)) as i16
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
