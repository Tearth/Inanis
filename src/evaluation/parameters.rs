// --------------------------------------------------- //
// Generated at 18-09-2022 12:28:18 UTC (e = 0.070485) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 407, 404, 609, 1246, 10000],
            piece_phase_value: [0, 1, 1, 2, 4, 0],
            initial_game_phase: 24,

            mobility_opening: [3, 6, 3, 5, 1, 3],
            mobility_ending: [6, 0, 1, 7, 5, 2],
            mobility_center_multiplier: [6, 0, 5, 1, 3, 4],

            doubled_pawn_opening: 1,
            doubled_pawn_ending: -9,

            isolated_pawn_opening: -25,
            isolated_pawn_ending: -5,

            chained_pawn_opening: 4,
            chained_pawn_ending: 7,

            passed_pawn_opening: 2,
            passed_pawn_ending: 56,

            pawn_shield_opening: 7,
            pawn_shield_ending: 2,

            pawn_shield_open_file_opening: -27,
            pawn_shield_open_file_ending: 4,

            king_attacked_squares_opening: -8,
            king_attacked_squares_ending: 3,

            pst: [[[[0; 64]; 2]; 6]; 2],
            pst_patterns: [[[0; 64]; 2]; 6],
        };

        evaluation_parameters.set_default_pst_patterns();
        evaluation_parameters.recalculate();
        evaluation_parameters
    }
}
