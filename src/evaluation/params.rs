// ------------------------------------------------------------------------- //
// Generated at 08-12-2024 21:33:08 UTC (e = 0.068001, k = 0.0077, r = 0.70) //
// ------------------------------------------------------------------------- //

use super::*;

pub const TEMPO: i16 = 15;
pub const BISHOP_PAIR: PackedEval = s!(19, 56);
pub const ROOK_OPEN_FILE: PackedEval = s!(24, -3);
pub const ROOK_SEMI_OPEN_FILE: PackedEval = s!(9, 14);
pub const MOBILITY_INNER: [PackedEval; 6] = [s!(0, 0), s!(10, 2), s!(9, 12), s!(7, 2), s!(3, 7), s!(0, 0)];
pub const MOBILITY_OUTER: [PackedEval; 6] = [s!(0, 0), s!(3, 0), s!(3, 0), s!(3, 0), s!(2, 0), s!(0, 0)];
pub const DOUBLED_PAWN: [PackedEval; 8] = [s!(-1, -8), s!(-15, -19), s!(-25, -41), s!(-54, -49), s!(-22, -39), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ISOLATED_PAWN: [PackedEval; 8] = [s!(1, -9), s!(-12, -20), s!(-24, -31), s!(-35, -39), s!(-37, -57), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const CHAINED_PAWN: [PackedEval; 8] = [s!(3, 1), s!(12, 8), s!(19, 18), s!(25, 32), s!(31, 49), s!(38, 62), s!(42, 42), s!(0, 0)];
pub const PASSED_PAWN: [PackedEval; 8] = [s!(8, -33), s!(15, 16), s!(20, 54), s!(32, 59), s!(43, 52), s!(22, 41), s!(25, 14), s!(37, 26)];
pub const BACKWARD_PAWN_OPEN_FILE: [PackedEval; 8] = [s!(36, 21), s!(23, 10), s!(10, -0), s!(-6, 9), s!(15, 43), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const BACKWARD_PAWN_CLOSED_FILE: [PackedEval; 8] = [s!(21, 19), s!(17, 15), s!(16, 7), s!(12, 7), s!(14, 41), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD: [PackedEval; 8] = [s!(2, 13), s!(13, 18), s!(20, 21), s!(21, 20), s!(66, 13), s!(20, 21), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD_OPEN_FILE: [PackedEval; 8] = [s!(-21, -21), s!(-22, -19), s!(-27, -20), s!(-34, -26), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const KING_AREA_THREATS: [PackedEval; 8] = [s!(-56, 45), s!(-58, 39), s!(-51, 37), s!(-34, 34), s!(-2, 18), s!(44, 0), s!(90, -22), s!(173, -61)];
