// --------------------------------------------------- //
// Generated at 07-02-2023 19:46:07 UTC (e = 0.082573) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 370, 381, 584, 1187, 10000],
            piece_phase_value: [0, 1, 1, 2, 4, 0],
            initial_game_phase: 24,

            mobility_opening: [5, 7, 3, 3, 1, 3],
            mobility_ending: [4, 0, 2, 5, 4, 3],
            mobility_center_multiplier: [5, 0, 4, 2, 3, 6],

            doubled_pawn_opening: 0,
            doubled_pawn_ending: -6,

            isolated_pawn_opening: -20,
            isolated_pawn_ending: -7,

            chained_pawn_opening: 4,
            chained_pawn_ending: 7,

            passed_pawn_opening: 2,
            passed_pawn_ending: 59,

            pawn_shield_opening: 8,
            pawn_shield_ending: -1,

            pawn_shield_open_file_opening: -28,
            pawn_shield_open_file_ending: 2,

            king_attacked_squares_opening: [-10, -12, -32, -55, -129, -170, -255, -306],
            king_attacked_squares_ending: [-37, -34, -22, -21, 8, 25, 56, 79],

            pst: [[[[0; 64]; 2]; 6]; 2],
            pst_patterns: [[[0; 64]; 2]; 6],
        };

        evaluation_parameters.set_default_pst_patterns();
        evaluation_parameters.recalculate();
        evaluation_parameters
    }
}
