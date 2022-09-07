// --------------------------------------------------- //
// Generated at 06-09-2022 17:45:06 UTC (e = 0.070550) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 403, 421, 608, 1253, 10000],
            initial_material: 9834,

            mobility_opening: [3, 2, 2, 5, 1, 6],
            mobility_ending: [4, 0, 0, 7, 6, 4],
            mobility_center_multiplier: [2, 1, 7, 1, 2, 5],

            doubled_pawn_opening: 1,
            doubled_pawn_ending: -7,

            isolated_pawn_opening: -25,
            isolated_pawn_ending: -6,

            chained_pawn_opening: 4,
            chained_pawn_ending: 9,

            passing_pawn_opening: -4,
            passing_pawn_ending: 68,

            pawn_shield_opening: 8,
            pawn_shield_ending: 3,

            pawn_shield_open_file_opening: -28,
            pawn_shield_open_file_ending: 5,

            king_attacked_fields_opening: -8,
            king_attacked_fields_ending: 3,

            pst: [[[[0; 64]; 2]; 6]; 2],
            pst_patterns: [[[0; 64]; 2]; 6],
        };

        evaluation_parameters.set_default_pst_patterns();
        evaluation_parameters.recalculate();
        evaluation_parameters
    }
}
