// ------------------------------------------------------------------------- //
// Generated at 08-09-2024 18:53:08 UTC (e = 0.114547, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

pub const BISHOP_PAIR: PackedEval = s!(22, 61);
pub const MOBILITY_INNER: [PackedEval; 6] = [s!(0, 0), s!(11, 3), s!(10, 12), s!(10, 3), s!(4, 8), s!(0, 0)];
pub const MOBILITY_OUTER: [PackedEval; 6] = [s!(0, 0), s!(3, 3), s!(2, 3), s!(5, 3), s!(1, 2), s!(0, 0)];
pub const DOUBLED_PAWN: [PackedEval; 8] = [s!(-6, -5), s!(-17, -18), s!(-24, -39), s!(-50, -54), s!(-21, -38), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ISOLATED_PAWN: [PackedEval; 8] = [s!(-3, -6), s!(-14, -17), s!(-24, -32), s!(-36, -40), s!(-29, -62), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const CHAINED_PAWN: [PackedEval; 8] = [s!(-5, 12), s!(5, 12), s!(17, 19), s!(26, 30), s!(34, 45), s!(40, 58), s!(51, 37), s!(20, 18)];
pub const PASSED_PAWN: [PackedEval; 8] = [s!(17, -36), s!(22, 9), s!(22, 61), s!(21, 68), s!(17, 46), s!(37, 42), s!(28, 15), s!(37, 26)];
pub const PAWN_SHIELD: [PackedEval; 8] = [s!(10, 10), s!(18, 16), s!(22, 19), s!(21, 22), s!(51, 23), s!(19, 20), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD_OPEN_FILE: [PackedEval; 8] = [s!(-16, -27), s!(-21, -20), s!(-33, -14), s!(-34, -25), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const KING_ATTACKED_SQUARES: [PackedEval; 8] = [s!(82, -49), s!(79, -44), s!(67, -39), s!(40, -36), s!(-7, -14), s!(-56, 3), s!(-112, 29), s!(-199, 64)];
