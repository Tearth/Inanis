use crate::state::*;

pub mod material;
pub mod mobility;
pub mod parameters;
pub mod pawns;
pub mod pst;
pub mod safety;

pub struct EvaluationParameters {
    pub piece_value: [i16; 6],

    pub piece_mobility_opening: [i16; 6],
    pub piece_mobility_ending: [i16; 6],
    pub piece_mobility_center_multiplier: [i16; 6],

    pub doubled_pawn_opening: i16,
    pub doubled_pawn_ending: i16,

    pub isolated_pawn_opening: i16,
    pub isolated_pawn_ending: i16,

    pub chained_pawn_opening: i16,
    pub chained_pawn_ending: i16,

    pub passing_pawn_opening: i16,
    pub passing_pawn_ending: i16,

    pub pawn_shield_opening: i16,
    pub pawn_shield_ending: i16,

    pub pawn_shield_open_file_opening: i16,
    pub pawn_shield_open_file_ending: i16,

    pub king_attacked_fields_opening: i16,
    pub king_attacked_fields_ending: i16,

    pub pst: [[[[i16; 64]; 2]; 2]; 6],
    pub pst_patterns: [[[i16; 64]; 2]; 6],
}

impl EvaluationParameters {
    fn set_default_pst_patterns(&mut self) {
        self.pst_patterns[PAWN as usize] = self.get_pawn_pst_pattern();
        self.pst_patterns[KNIGHT as usize] = self.get_knight_pst_pattern();
        self.pst_patterns[BISHOP as usize] = self.get_bishop_pst_pattern();
        self.pst_patterns[ROOK as usize] = self.get_rook_pst_pattern();
        self.pst_patterns[QUEEN as usize] = self.get_queen_pst_pattern();
        self.pst_patterns[KING as usize] = self.get_king_pst_pattern();
    }

    pub fn recalculate(&mut self) {
        for piece in [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING] {
            for color in [WHITE, BLACK] {
                for phase in [OPENING, ENDING] {
                    self.pst[piece as usize][color as usize][phase as usize] =
                        self.calculate_pst_pattern(color, &self.pst_patterns[piece as usize][phase as usize]);
                }
            }
        }
    }

    fn calculate_pst_pattern(&self, color: u8, pattern: &[i16; 64]) -> [i16; 64] {
        let mut array = [0; 64];

        match color {
            WHITE => {
                for field_index in 0..64 {
                    array[field_index] = pattern[63 - field_index];
                }
            }
            BLACK => {
                for file in 0..8 {
                    for rank in 0..8 {
                        array[file + rank * 8] = pattern[(7 - file) + rank * 8];
                    }
                }
            }
            _ => panic!("Invalid value: color={}", color),
        }

        array
    }

    pub fn get_pst_value(&self, piece: u8, color: u8, phase: u8, field: u8) -> i16 {
        unsafe { self.pst[piece as usize][color as usize][phase as usize][field as usize] as i16 }
    }

    pub fn get_initial_material(&self) -> i16 {
        16 * self.piece_value[PAWN as usize]
            + 4 * self.piece_value[KNIGHT as usize]
            + 4 * self.piece_value[BISHOP as usize]
            + 4 * self.piece_value[ROOK as usize]
            + 2 * self.piece_value[QUEEN as usize]
    }
}

/// Blends `opening_score` and `ending_score` with the ratio passed in `game_phase`. The ratio is a number from 0.0 to 1.0, where:
///  - 1.0 represents a board with the initial state set (opening phase)
///  - 0.0 represents a board without any piece (ending phase)
///  - every value between them represents a board state somewhere in the middle game
pub fn taper_score(game_phase: f32, opening_score: i16, ending_score: i16) -> i16 {
    ((game_phase * (opening_score as f32)) + ((1.0 - game_phase) * (ending_score as f32))) as i16
}
