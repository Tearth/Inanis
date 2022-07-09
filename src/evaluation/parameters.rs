// --------------------------------------------------- //
// Generated at 2022-07-08 11:00:32 UTC (e = 0.082809) //
// --------------------------------------------------- //

use super::*;

impl Default for EvaluationParameters {
    fn default() -> Self {
        let mut evaluation_parameters = Self {
            piece_value: [100, 387, 420, 604, 1245, 10000],
            initial_material: 9734,

            mobility_opening: [3, 4, 3, 5, 1, 6],
            mobility_ending: [3, 1, 0, 6, 3, 6],
            mobility_center_multiplier: [3, 2, 4, 1, 3, 5],

            doubled_pawn_opening: 0,
            doubled_pawn_ending: -11,

            isolated_pawn_opening: -18,
            isolated_pawn_ending: -9,

            chained_pawn_opening: 5,
            chained_pawn_ending: 7,

            passing_pawn_opening: -10,
            passing_pawn_ending: 64,

            pawn_shield_opening: 7,
            pawn_shield_ending: 3,

            pawn_shield_open_file_opening: -26,
            pawn_shield_open_file_ending: 6,

            king_attacked_fields_opening: -22,
            king_attacked_fields_ending: 7,

            pst: [[[[0; 64]; 2]; 6]; 2],
            pst_patterns: [[[0; 64]; 2]; 6],
        };

        evaluation_parameters.set_default_pst_patterns();
        evaluation_parameters.recalculate();
        evaluation_parameters
    }
}
