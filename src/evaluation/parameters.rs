// --------------------------------------------------- //
// Generated at 09-08-2024 10:19:09 UTC (e = 0.114457) //
// --------------------------------------------------- //

use super::*;

pub const INITIAL_GAME_PHASE: u8 = 24;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 337, 338, 521, 1050, 10000],

            bishop_pair_opening: 22,
            bishop_pair_ending: 61,

            mobility_inner_opening: [0, 11, 10, 10, 4, 0],
            mobility_inner_ending: [0, 3, 12, 3, 8, 0],

            mobility_outer_opening: [0, 3, 2, 5, 1, 0],
            mobility_outer_ending: [0, 3, 3, 3, 2, 0],

            doubled_pawn_opening: [-6, -18, -24, -49, -21, 0, 0, 0],
            doubled_pawn_ending: [-5, -18, -39, -54, -38, 0, 0, 0],

            isolated_pawn_opening: [-3, -14, -24, -36, -29, 0, 0, 0],
            isolated_pawn_ending: [-6, -17, -32, -40, -62, 0, 0, 0],

            chained_pawn_opening: [-5, 5, 17, 26, 34, 40, 51, 20],
            chained_pawn_ending: [11, 12, 19, 30, 45, 58, 37, 18],

            passed_pawn_opening: [18, 22, 22, 21, 17, 37, 28, 37],
            passed_pawn_ending: [-36, 8, 61, 68, 46, 42, 15, 26],

            pawn_shield_opening: [10, 18, 22, 21, 51, 19, 0, 0],
            pawn_shield_ending: [10, 16, 19, 22, 23, 20, 0, 0],

            pawn_shield_open_file_opening: [-16, -21, -33, -34, 0, 0, 0, 0],
            pawn_shield_open_file_ending: [-27, -20, -14, -25, 0, 0, 0, 0],

            king_attacked_squares_opening: [82, 79, 67, 40, -7, -56, -112, -200],
            king_attacked_squares_ending: [-49, -44, -39, -36, -14, 3, 29, 65],

            pst: Box::new([[[[[0; 64]; 2]; 8]; 6]; 2]),
            pst_patterns: Box::new([[[[0; 64]; 2]; 8]; 6]),

            piece_phase_value: [0, 1, 1, 2, 4, 0],
            initial_game_phase: 24,
        };

        evaluation_parameters.set_default_pst_patterns();
        evaluation_parameters.recalculate();
        evaluation_parameters
    }
}
