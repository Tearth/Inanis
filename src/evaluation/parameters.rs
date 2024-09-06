// ------------------------------------------------------------------------- //
// Generated at 06-09-2024 19:43:06 UTC (e = 0.114563, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        Self {
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
            passed_pawn_ending: [-36, 9, 61, 68, 46, 42, 15, 26],

            pawn_shield_opening: [10, 18, 22, 21, 51, 19, 0, 0],
            pawn_shield_ending: [10, 16, 19, 22, 23, 20, 0, 0],

            pawn_shield_open_file_opening: [-16, -21, -33, -34, 0, 0, 0, 0],
            pawn_shield_open_file_ending: [-27, -20, -14, -25, 0, 0, 0, 0],

            king_attacked_squares_opening: [82, 79, 67, 40, -7, -56, -112, -200],
            king_attacked_squares_ending: [-49, -44, -39, -36, -14, 3, 29, 65],
        }
    }
}
