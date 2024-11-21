// ------------------------------------------------------------------------- //
// Generated at 21-11-2024 07:47:21 UTC (e = 0.112638, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

pub const BISHOP_PAIR: PackedEval = s!(24, 60);
pub const MOBILITY_INNER: [PackedEval; 6] = [s!(0, 0), s!(11, 3), s!(10, 13), s!(11, 3), s!(4, 8), s!(0, 0)];
pub const MOBILITY_OUTER: [PackedEval; 6] = [s!(0, 0), s!(3, 2), s!(3, 2), s!(5, 2), s!(2, 3), s!(0, 0)];
pub const DOUBLED_PAWN: [PackedEval; 8] = [s!(-1, -3), s!(-14, -17), s!(-23, -36), s!(-58, -59), s!(-22, -39), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ISOLATED_PAWN: [PackedEval; 8] = [s!(-1, -8), s!(-13, -19), s!(-24, -33), s!(-37, -39), s!(-31, -59), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const CHAINED_PAWN: [PackedEval; 8] = [s!(-2, 7), s!(9, 9), s!(18, 17), s!(27, 30), s!(33, 46), s!(39, 59), s!(47, 44), s!(17, 18)];
pub const PASSED_PAWN: [PackedEval; 8] = [s!(17, -33), s!(22, 11), s!(21, 55), s!(29, 66), s!(27, 53), s!(25, 42), s!(25, 13), s!(37, 26)];
pub const PAWN_SHIELD: [PackedEval; 8] = [s!(12, 12), s!(20, 14), s!(23, 15), s!(20, 19), s!(48, 28), s!(19, 20), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD_OPEN_FILE: [PackedEval; 8] = [s!(-21, -17), s!(-22, -18), s!(-28, -20), s!(-33, -30), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const KING_AREA_THREATS: [PackedEval; 8] = [s!(-68, 41), s!(-69, 40), s!(-60, 37), s!(-37, 32), s!(4, 16), s!(52, -2), s!(104, -24), s!(180, -51)];
