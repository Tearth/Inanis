// ------------------------------------------------------------------------- //
// Generated at 20-09-2024 15:32:20 UTC (e = 0.114601, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

pub const BISHOP_PAIR: PackedEval = s!(22, 61);
pub const MOBILITY_INNER: [PackedEval; 6] = [s!(0, 0), s!(11, 3), s!(10, 12), s!(10, 3), s!(4, 8), s!(0, 0)];
pub const MOBILITY_OUTER: [PackedEval; 6] = [s!(0, 0), s!(3, 2), s!(2, 3), s!(5, 2), s!(1, 2), s!(0, 0)];
pub const DOUBLED_PAWN: [PackedEval; 8] = [s!(-6, -1), s!(-17, -19), s!(-24, -41), s!(-52, -55), s!(-21, -38), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ISOLATED_PAWN: [PackedEval; 8] = [s!(-2, -8), s!(-14, -18), s!(-25, -32), s!(-37, -39), s!(-29, -61), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const CHAINED_PAWN: [PackedEval; 8] = [s!(-3, 11), s!(8, 12), s!(17, 19), s!(26, 30), s!(33, 45), s!(39, 58), s!(48, 37), s!(20, 18)];
pub const PASSED_PAWN: [PackedEval; 8] = [s!(16, -33), s!(20, 11), s!(19, 57), s!(25, 66), s!(19, 49), s!(37, 42), s!(28, 15), s!(37, 26)];
pub const PAWN_SHIELD: [PackedEval; 8] = [s!(11, 9), s!(18, 15), s!(23, 18), s!(19, 23), s!(51, 23), s!(19, 20), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD_OPEN_FILE: [PackedEval; 8] = [s!(-15, -27), s!(-22, -20), s!(-33, -14), s!(-34, -25), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const KING_AREA_THREATS: [PackedEval; 8] = [s!(-82, 49), s!(-79, 44), s!(-67, 39), s!(-40, 36), s!(7, 14), s!(56, -3), s!(112, -29), s!(198, -64)];
