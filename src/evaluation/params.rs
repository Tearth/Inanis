// ------------------------------------------------------------------------- //
// Generated at 19-11-2024 23:40:19 UTC (e = 0.113270, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

pub const BISHOP_PAIR: PackedEval = s!(24, 60);
pub const MOBILITY_INNER: [PackedEval; 6] = [s!(0, 0), s!(11, 3), s!(10, 12), s!(10, 2), s!(4, 8), s!(0, 0)];
pub const MOBILITY_OUTER: [PackedEval; 6] = [s!(0, 0), s!(3, 2), s!(3, 3), s!(5, 2), s!(2, 2), s!(0, 0)];
pub const DOUBLED_PAWN: [PackedEval; 8] = [s!(-3, 0), s!(-15, -17), s!(-23, -40), s!(-57, -59), s!(-21, -38), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ISOLATED_PAWN: [PackedEval; 8] = [s!(-1, -10), s!(-13, -20), s!(-24, -32), s!(-38, -38), s!(-31, -58), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const CHAINED_PAWN: [PackedEval; 8] = [s!(-3, 9), s!(9, 11), s!(18, 19), s!(26, 30), s!(33, 45), s!(39, 58), s!(47, 39), s!(19, 18)];
pub const PASSED_PAWN: [PackedEval; 8] = [s!(16, -30), s!(21, 13), s!(18, 58), s!(27, 65), s!(24, 49), s!(32, 38), s!(27, 14), s!(37, 26)];
pub const PAWN_SHIELD: [PackedEval; 8] = [s!(12, 9), s!(19, 14), s!(23, 17), s!(19, 21), s!(50, 27), s!(19, 20), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD_OPEN_FILE: [PackedEval; 8] = [s!(-20, -24), s!(-23, -18), s!(-30, -16), s!(-31, -28), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const KING_AREA_THREATS: [PackedEval; 8] = [s!(-72, 42), s!(-73, 40), s!(-62, 36), s!(-39, 32), s!(4, 15), s!(53, -1), s!(106, -24), s!(188, -53)];
