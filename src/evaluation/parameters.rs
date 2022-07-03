// --------------------------------------------------- //
// Generated at 2022-07-03 13:23:55 UTC (e = 0.070760) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 381, 421, 632, 1262, 10000],
            initial_material: 9860,

            mobility_opening: [2, 4, 4, 4, 0, 2],
            mobility_ending: [6, 2, 2, 5, 5, 5],
            mobility_center_multiplier: [4, 3, 3, 2, 3, 4],

            doubled_pawn_opening: 1,
            doubled_pawn_ending: -14,

            isolated_pawn_opening: -22,
            isolated_pawn_ending: -8,

            chained_pawn_opening: 5,
            chained_pawn_ending: 6,

            passing_pawn_opening: -5,
            passing_pawn_ending: 58,

            pawn_shield_opening: 10,
            pawn_shield_ending: 3,

            pawn_shield_open_file_opening: -25,
            pawn_shield_open_file_ending: 6,

            king_attacked_fields_opening: -25,
            king_attacked_fields_ending: 9,

            pst: [[[[0; 64]; 2]; 6]; 2],
            pst_patterns: [[[0; 64]; 2]; 6],
        };

        evaluation_parameters.set_default_pst_patterns();
        evaluation_parameters.recalculate();
        evaluation_parameters
    }
}
