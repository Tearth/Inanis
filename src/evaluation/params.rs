// ------------------------------------------------------------------------- //
// Generated at 24-11-2024 15:06:24 UTC (e = 0.130552, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

pub const BISHOP_PAIR: PackedEval = s!(18, 53);
pub const MOBILITY_INNER: [PackedEval; 6] = [s!(0, 0), s!(11, 0), s!(9, 12), s!(10, 1), s!(4, 7), s!(0, 0)];
pub const MOBILITY_OUTER: [PackedEval; 6] = [s!(0, 0), s!(3, 0), s!(3, 0), s!(5, 0), s!(2, 0), s!(0, 0)];
pub const DOUBLED_PAWN: [PackedEval; 8] = [s!(-4, -9), s!(-16, -18), s!(-22, -39), s!(-53, -49), s!(-22, -39), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ISOLATED_PAWN: [PackedEval; 8] = [s!(-4, -12), s!(-14, -21), s!(-23, -31), s!(-32, -38), s!(-33, -55), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const CHAINED_PAWN: [PackedEval; 8] = [s!(3, 5), s!(12, 9), s!(19, 18), s!(25, 31), s!(31, 47), s!(38, 59), s!(43, 42), s!(0, 0)];
pub const PASSED_PAWN: [PackedEval; 8] = [s!(11, -36), s!(16, 15), s!(19, 56), s!(30, 61), s!(43, 54), s!(22, 41), s!(25, 14), s!(37, 26)];
pub const PAWN_SHIELD: [PackedEval; 8] = [s!(0, 13), s!(13, 18), s!(21, 21), s!(21, 19), s!(67, 15), s!(20, 21), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD_OPEN_FILE: [PackedEval; 8] = [s!(-20, -23), s!(-21, -20), s!(-28, -19), s!(-35, -24), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const KING_AREA_THREATS: [PackedEval; 8] = [s!(-63, 47), s!(-64, 42), s!(-56, 39), s!(-37, 35), s!(1, 14), s!(49, -4), s!(95, -24), s!(180, -61)];
