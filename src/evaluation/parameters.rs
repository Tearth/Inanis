// --------------------------------------------------- //
// Generated at 2022-05-13 18:52:01 UTC (e = 0.064056) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 420, 444, 650, 1326, 10000],
            initial_material: 10308,

            mobility_opening: [5, 8, 5, 6, 2, 6],
            mobility_ending: [3, 0, 0, 5, 6, 3],
            mobility_center_multiplier: [6, 1, 3, 1, 1, 6],

            doubled_pawn_opening: 5,
            doubled_pawn_ending: -12,

            isolated_pawn_opening: -31,
            isolated_pawn_ending: 1,

            chained_pawn_opening: 5,
            chained_pawn_ending: 11,

            passing_pawn_opening: -1,
            passing_pawn_ending: 57,

            pawn_shield_opening: 12,
            pawn_shield_ending: 5,

            pawn_shield_open_file_opening: -27,
            pawn_shield_open_file_ending: 2,

            king_attacked_fields_opening: -19,
            king_attacked_fields_ending: 6,

            pst: [[[[0; 64]; 2]; 6]; 2],
            pst_patterns: [[[0; 64]; 2]; 6],
        };

        evaluation_parameters.set_default_pst_patterns();
        evaluation_parameters.recalculate();
        evaluation_parameters
    }
}
