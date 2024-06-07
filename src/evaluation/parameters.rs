// --------------------------------------------------- //
// Generated at 05-06-2024 21:38:05 UTC (e = 0.081972) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 352, 358, 543, 1071, 10000],

            bishop_pair_opening: 19,
            bishop_pair_ending: 46,

            mobility_inner_opening: [5, 10, 10, 9, 2, 4],
            mobility_inner_ending: [4, 1, 7, 3, 11, 3],

            mobility_outer_opening: [6, 3, 3, 5, 1, 4],
            mobility_outer_ending: [6, 0, 2, 4, 3, 5],

            doubled_pawn_opening: [-17, -20, -24, -36, -21, -38, -24, -36],
            doubled_pawn_ending: [-11, -26, -44, -37, -38, -19, -23, -19],

            isolated_pawn_opening: [9, -9, -23, -35, -48, -38, -11, -23],
            isolated_pawn_ending: [-11, -20, -31, -41, -54, -34, -23, -14],

            chained_pawn_opening: [4, 13, 21, 28, 33, 35, 32, 20],
            chained_pawn_ending: [8, 12, 23, 37, 53, 53, 27, 18],

            passed_pawn_opening: [15, 20, 18, 21, 34, 28, 28, 37],
            passed_pawn_ending: [-33, 12, 56, 67, 63, 19, 17, 26],

            pawn_shield_opening: [9, 20, 30, 31, 32, 19, 25, 29],
            pawn_shield_ending: [17, 20, 16, 10, 32, 18, 34, 17],

            pawn_shield_open_file_opening: [8, -15, -40, -58, -13, -22, -12, -33],
            pawn_shield_open_file_ending: [-30, -19, -16, -22, -22, -40, -36, -10],

            king_attacked_squares_opening: [99, 94, 77, 54, -10, -53, -132, -235],
            king_attacked_squares_ending: [-57, -53, -43, -41, -14, 3, 37, 84],

            pst: [[[[0; 64]; 2]; 6]; 2],
            pst_patterns: [[[0; 64]; 2]; 6],

            piece_phase_value: [0, 1, 1, 2, 4, 0],
            initial_game_phase: 24,
        };

        evaluation_parameters.set_default_pst_patterns();
        evaluation_parameters.recalculate();
        evaluation_parameters
    }
}
