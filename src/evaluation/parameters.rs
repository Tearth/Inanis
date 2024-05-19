// --------------------------------------------------- //
// Generated at 17-05-2024 12:13:17 UTC (e = 0.082249) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 349, 359, 538, 1050, 10000],
            piece_phase_value: [0, 1, 1, 2, 4, 0],
            initial_game_phase: 24,

            mobility_inner_opening: [4, 2, 8, 6, 1, 3],
            mobility_inner_ending: [3, 1, 9, 4, 12, 2],

            mobility_outer_opening: [5, 2, 2, 4, 1, 3],
            mobility_outer_ending: [5, 0, 2, 4, 3, 4],

            doubled_pawn_opening: -1,
            doubled_pawn_ending: -7,

            isolated_pawn_opening: -17,
            isolated_pawn_ending: -7,

            chained_pawn_opening: 4,
            chained_pawn_ending: 4,

            passed_pawn_opening: 7,
            passed_pawn_ending: 41,

            pawn_shield_opening: 6,
            pawn_shield_ending: -1,

            pawn_shield_open_file_opening: -20,
            pawn_shield_open_file_ending: 2,

            king_attacked_squares_opening: [98, 93, 76, 54, -12, -53, -133, -233],
            king_attacked_squares_ending: [-58, -53, -43, -42, -13, 3, 37, 80],

            pst: [[[[0; 64]; 2]; 6]; 2],
            pst_patterns: [[[0; 64]; 2]; 6],
        };

        evaluation_parameters.set_default_pst_patterns();
        evaluation_parameters.recalculate();
        evaluation_parameters
    }
}
