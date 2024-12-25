// ------------------------------------------------------------------------- //
// Generated at 23-12-2024 19:19:23 UTC (e = 0.067721, k = 0.0077, r = 0.70) //
// ------------------------------------------------------------------------- //

use super::*;

pub const TEMPO: i16 = 15;
pub const BISHOP_PAIR: PackedEval = s!(21, 55);
pub const PAWNS_ATTACKING_PIECES: PackedEval = s!(42, 40);
pub const ROOK_OPEN_FILE: PackedEval = s!(24, -3);
pub const ROOK_SEMI_OPEN_FILE: PackedEval = s!(9, 14);
pub const MOBILITY_INNER: [PackedEval; 6] = [s!(0, 0), s!(11, 2), s!(10, 12), s!(7, 2), s!(4, 7), s!(0, 0)];
pub const MOBILITY_OUTER: [PackedEval; 6] = [s!(0, 0), s!(3, 0), s!(3, 0), s!(3, 0), s!(2, 0), s!(0, 0)];
pub const DOUBLED_PAWN: [PackedEval; 8] = [s!(-1, -8), s!(-15, -20), s!(-25, -41), s!(-54, -49), s!(-22, -39), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ISOLATED_PAWN: [PackedEval; 8] = [s!(-0, -9), s!(-13, -20), s!(-23, -31), s!(-34, -39), s!(-37, -56), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const CHAINED_PAWN: [PackedEval; 8] = [s!(1, 1), s!(11, 8), s!(19, 18), s!(26, 32), s!(32, 49), s!(39, 62), s!(43, 42), s!(0, 0)];
pub const PASSED_PAWN: [PackedEval; 8] = [s!(10, -33), s!(15, 16), s!(19, 55), s!(31, 59), s!(42, 51), s!(22, 41), s!(25, 14), s!(37, 26)];
pub const BACKWARD_PAWN_OPEN_FILE: [PackedEval; 8] = [s!(36, 21), s!(23, 10), s!(10, 0), s!(-6, 9), s!(15, 43), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const BACKWARD_PAWN_CLOSED_FILE: [PackedEval; 8] = [s!(21, 19), s!(17, 16), s!(16, 7), s!(13, 6), s!(14, 41), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD: [PackedEval; 8] = [s!(1, 14), s!(13, 18), s!(21, 20), s!(22, 20), s!(66, 13), s!(20, 21), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD_OPEN_FILE: [PackedEval; 8] = [s!(-22, -21), s!(-22, -19), s!(-27, -20), s!(-33, -26), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const KING_AREA_THREATS: [PackedEval; 8] = [s!(-55, 44), s!(-57, 39), s!(-50, 37), s!(-33, 34), s!(-1, 17), s!(43, 0), s!(89, -21), s!(170, -60)];
