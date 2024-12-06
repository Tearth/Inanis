// ------------------------------------------------------------------------- //
// Generated at 05-12-2024 20:56:05 UTC (e = 0.130216, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

pub const BISHOP_PAIR: PackedEval = s!(18, 53);
pub const MOBILITY_INNER: [PackedEval; 6] = [s!(0, 0), s!(11, 0), s!(9, 12), s!(10, 1), s!(3, 7), s!(0, 0)];
pub const MOBILITY_OUTER: [PackedEval; 6] = [s!(0, 0), s!(3, 0), s!(3, 0), s!(5, 0), s!(2, 0), s!(0, 0)];
pub const DOUBLED_PAWN: [PackedEval; 8] = [s!(-4, -8), s!(-15, -19), s!(-23, -40), s!(-53, -49), s!(-22, -39), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ISOLATED_PAWN: [PackedEval; 8] = [s!(-1, -9), s!(-13, -20), s!(-24, -31), s!(-33, -40), s!(-35, -57), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const CHAINED_PAWN: [PackedEval; 8] = [s!(2, 2), s!(12, 9), s!(19, 19), s!(25, 32), s!(32, 48), s!(39, 59), s!(42, 42), s!(0, 0)];
pub const PASSED_PAWN: [PackedEval; 8] = [s!(11, -36), s!(16, 16), s!(19, 56), s!(30, 61), s!(43, 53), s!(22, 41), s!(25, 14), s!(37, 26)];
pub const BACKWARD_PAWN_OPEN_FILE: [PackedEval; 8] = [s!(38, 24), s!(22, 11), s!(8, -5), s!(-5, 10), s!(15, 43), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const BACKWARD_PAWN_CLOSED_FILE: [PackedEval; 8] = [s!(22, 17), s!(17, 13), s!(17, 3), s!(9, 12), s!(15, 42), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD: [PackedEval; 8] = [s!(0, 13), s!(13, 18), s!(21, 21), s!(21, 20), s!(67, 14), s!(20, 21), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD_OPEN_FILE: [PackedEval; 8] = [s!(-20, -23), s!(-21, -20), s!(-27, -19), s!(-35, -24), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const KING_AREA_THREATS: [PackedEval; 8] = [s!(-63, 47), s!(-64, 42), s!(-56, 39), s!(-37, 35), s!(1, 15), s!(49, -4), s!(95, -24), s!(180, -61)];
