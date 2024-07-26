// --------------------------------------------------- //
// Generated at 25-07-2024 18:43:25 UTC (e = 0.080876) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 356, 363, 545, 1076, 10000],

            bishop_pair_opening: 19,
            bishop_pair_ending: 47,

            mobility_inner_opening: [5, 10, 10, 9, 3, 4],
            mobility_inner_ending: [4, 1, 8, 3, 11, 3],

            mobility_outer_opening: [6, 3, 3, 5, 1, 4],
            mobility_outer_ending: [6, 0, 2, 3, 3, 5],

            doubled_pawn_opening: [-16, -21, -25, -36, -21, -38, -24, -36],
            doubled_pawn_ending: [-8, -24, -45, -41, -38, -19, -23, -19],

            isolated_pawn_opening: [9, -10, -23, -34, -48, -38, -11, -23],
            isolated_pawn_ending: [-12, -20, -31, -41, -54, -34, -23, -14],

            chained_pawn_opening: [3, 13, 21, 28, 32, 35, 34, 20],
            chained_pawn_ending: [6, 10, 21, 36, 54, 57, 28, 18],

            passed_pawn_opening: [7, 15, 19, 21, 36, 37, 28, 37],
            passed_pawn_ending: [-34, 10, 55, 65, 62, 27, 17, 26],

            pawn_shield_opening: [-4, 14, 27, 33, 51, 19, 25, 29],
            pawn_shield_ending: [12, 16, 15, 12, 36, 20, 34, 17],

            pawn_shield_open_file_opening: [-14, -24, -33, -33, -13, -22, -12, -33],
            pawn_shield_open_file_ending: [-27, -19, -18, -23, -22, -40, -36, -10],

            king_attacked_squares_opening: [92, 87, 72, 52, -7, -47, -123, -231],
            king_attacked_squares_ending: [-55, -51, -42, -40, -15, 1, 33, 84],

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
