// --------------------------------------------------- //
// Generated at 28-05-2024 16:45:28 UTC (e = 0.082209) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 353, 360, 533, 1051, 10000],
            piece_phase_value: [0, 1, 1, 2, 4, 0],
            initial_game_phase: 24,

            mobility_inner_opening: [5, 2, 8, 6, 1, 4],
            mobility_inner_ending: [4, 0, 9, 4, 12, 3],

            mobility_outer_opening: [6, 1, 2, 4, 1, 4],
            mobility_outer_ending: [6, 0, 2, 4, 3, 5],

            doubled_pawn_opening: [-16, -19, -24, -36, -21, -38, -24, -36],
            doubled_pawn_ending: [-12, -26, -45, -36, -38, -19, -23, -19],

            isolated_pawn_opening: [8, -10, -24, -37, -45, -38, -11, -23],
            isolated_pawn_ending: [-11, -20, -30, -40, -55, -34, -23, -14],

            chained_pawn_opening: [3, 13, 21, 29, 33, 38, 28, 20],
            chained_pawn_ending: [10, 14, 24, 39, 55, 44, 27, 18],

            passed_pawn_opening: [11, 18, 20, 31, 30, 25, 28, 37],
            passed_pawn_ending: [-28, 15, 59, 66, 56, 16, 17, 26],

            pawn_shield_opening: [10, 23, 32, 33, 24, 19, 25, 29],
            pawn_shield_ending: [18, 21, 17, 11, 26, 18, 34, 17],

            pawn_shield_open_file_opening: [6, -16, -39, -55, -13, -22, -12, -33],
            pawn_shield_open_file_ending: [-29, -19, -16, -23, -22, -40, -36, -10],

            king_attacked_squares_opening: [98, 94, 77, 54, -10, -53, -132, -233],
            king_attacked_squares_ending: [-58, -53, -43, -42, -13, 4, 38, 83],

            pst: [[[[0; 64]; 2]; 6]; 2],
            pst_patterns: [[[0; 64]; 2]; 6],
        };

        evaluation_parameters.set_default_pst_patterns();
        evaluation_parameters.recalculate();
        evaluation_parameters
    }
}
