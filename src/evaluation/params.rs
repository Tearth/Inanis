// ------------------------------------------------------------------------- //
// Generated at 06-12-2024 18:23:06 UTC (e = 0.130063, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

pub const BISHOP_PAIR: PackedEval = s!(18, 53);
pub const ROOK_OPEN_FILE: PackedEval = s!(28, -4);
pub const ROOK_SEMI_OPEN_FILE: PackedEval = s!(10, 15);
pub const MOBILITY_INNER: [PackedEval; 6] = [s!(0, 0), s!(11, 0), s!(9, 12), s!(7, 1), s!(3, 7), s!(0, 0)];
pub const MOBILITY_OUTER: [PackedEval; 6] = [s!(0, 0), s!(3, 0), s!(3, 0), s!(3, 0), s!(2, 0), s!(0, 0)];
pub const DOUBLED_PAWN: [PackedEval; 8] = [s!(-2, -7), s!(-15, -19), s!(-24, -42), s!(-54, -49), s!(-22, -39), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ISOLATED_PAWN: [PackedEval; 8] = [s!(-0, -10), s!(-13, -20), s!(-24, -31), s!(-33, -39), s!(-36, -57), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const CHAINED_PAWN: [PackedEval; 8] = [s!(3, 2), s!(12, 8), s!(19, 18), s!(25, 32), s!(31, 48), s!(39, 61), s!(42, 42), s!(0, 0)];
pub const PASSED_PAWN: [PackedEval; 8] = [s!(9, -37), s!(16, 15), s!(19, 56), s!(31, 61), s!(44, 54), s!(22, 41), s!(25, 14), s!(37, 26)];
pub const BACKWARD_PAWN_OPEN_FILE: [PackedEval; 8] = [s!(39, 23), s!(23, 10), s!(7, -3), s!(-6, 9), s!(15, 43), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const BACKWARD_PAWN_CLOSED_FILE: [PackedEval; 8] = [s!(22, 19), s!(17, 15), s!(16, 5), s!(11, 8), s!(14, 41), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD: [PackedEval; 8] = [s!(-0, 13), s!(13, 18), s!(21, 21), s!(21, 20), s!(67, 13), s!(20, 21), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD_OPEN_FILE: [PackedEval; 8] = [s!(-20, -23), s!(-21, -20), s!(-27, -19), s!(-36, -24), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const KING_AREA_THREATS: [PackedEval; 8] = [s!(-61, 46), s!(-62, 41), s!(-55, 39), s!(-37, 35), s!(-0, 15), s!(48, -3), s!(94, -23), s!(179, -60)];
