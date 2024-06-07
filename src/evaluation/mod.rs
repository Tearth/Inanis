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

    pub pst: [[[[i16; 64]; 2]; 6]; 2],
    pub pst_patterns: [[[i16; 64]; 2]; 6],

    pub piece_phase_value: [u8; 6],
    pub initial_game_phase: u8,
}

pub struct EvaluationResult {
    pub opening_score: i16,
    pub ending_score: i16,
}

impl EvaluationParameters {
    /// Initializes PST patterns with used by default during search.
    fn set_default_pst_patterns(&mut self) {
        self.pst_patterns[PAWN] = self.get_pawn_pst_pattern();
        self.pst_patterns[KNIGHT] = self.get_knight_pst_pattern();
        self.pst_patterns[BISHOP] = self.get_bishop_pst_pattern();
        self.pst_patterns[ROOK] = self.get_rook_pst_pattern();
        self.pst_patterns[QUEEN] = self.get_queen_pst_pattern();
        self.pst_patterns[KING] = self.get_king_pst_pattern();
    }

    /// Recalculates initial material and PST tables.
    pub fn recalculate(&mut self) {
        for color in ALL_COLORS {
            for piece in ALL_PIECES {
                for phase in ALL_PHASES {
                    self.pst[color][piece][phase] = self.calculate_pst(color, &self.pst_patterns[piece][phase]);
                }
            }
        }
    }

    /// Calculates PST table for the specified `color` and `pattern`.
    fn calculate_pst(&self, color: usize, pattern: &[i16; 64]) -> [i16; 64] {
        let mut array = [0; 64];

        match color {
            WHITE => {
                for square in ALL_SQUARES {
                    array[square] = pattern[63 - square];
                }
            }
            BLACK => {
                for file in ALL_FILES {
                    for rank in ALL_RANKS {
                        array[file + rank * 8] = pattern[(7 - file) + rank * 8];
                    }
                }
            }
            _ => panic!("Invalid parameter: color={}", color),
        }

        array
    }

    /// Gets a PST value for the specified `color`, `piece`, `phase` and `square`.
    pub fn get_pst_value(&self, color: usize, piece: usize, phase: usize, square: usize) -> i16 {
        self.pst[color][piece][phase][square]
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
