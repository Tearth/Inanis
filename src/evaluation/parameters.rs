// --------------------------------------------------- //
// Generated at 27-07-2024 15:56:27 UTC (e = 0.076543) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 339, 344, 534, 1065, 10000],

            bishop_pair_opening: 23,
            bishop_pair_ending: 70,

            mobility_inner_opening: [5, 12, 12, 9, 3, 4],
            mobility_inner_ending: [4, 5, 10, 5, 10, 3],

            mobility_outer_opening: [6, 4, 3, 4, 1, 4],
            mobility_outer_ending: [6, 5, 4, 5, 4, 5],

            doubled_pawn_opening: [-13, -19, -26, -39, -21, -38, -24, -36],
            doubled_pawn_ending: [-5, -22, -44, -47, -38, -19, -23, -19],

            isolated_pawn_opening: [3, -12, -24, -36, -38, -38, -11, -23],
            isolated_pawn_ending: [-6, -17, -32, -42, -61, -34, -23, -14],

            chained_pawn_opening: [-5, 5, 16, 25, 32, 41, 52, 20],
            chained_pawn_ending: [6, 8, 18, 35, 51, 61, 33, 18],

            passed_pawn_opening: [18, 21, 23, 16, 18, 39, 28, 37],
            passed_pawn_ending: [-33, 9, 50, 68, 50, 42, 17, 26],

            pawn_shield_opening: [-3, 10, 23, 29, 62, 19, 25, 29],
            pawn_shield_ending: [10, 16, 18, 17, 30, 20, 34, 17],

            pawn_shield_open_file_opening: [-9, -21, -35, -39, -13, -22, -12, -33],
            pawn_shield_open_file_ending: [-29, -20, -15, -23, -22, -40, -36, -10],

            king_attacked_squares_opening: [84, 81, 69, 45, -1, -46, -115, -223],
            king_attacked_squares_ending: [-46, -43, -36, -35, -23, -6, 25, 80],

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
