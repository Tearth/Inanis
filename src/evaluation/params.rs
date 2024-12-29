// ------------------------------------------------------------------------- //
// Generated at 28-12-2024 13:59:28 UTC (e = 0.067562, k = 0.0077, r = 0.70) //
// ------------------------------------------------------------------------- //

use super::*;

pub const TEMPO: i16 = 15;
pub const BISHOP_PAIR: PackedEval = s!(20, 55);
pub const PAWNS_ATTACKING_PIECES: PackedEval = s!(42, 43);
pub const ROOK_OPEN_FILE: PackedEval = s!(24, -4);
pub const ROOK_SEMI_OPEN_FILE: PackedEval = s!(9, 15);
pub const MOBILITY_INNER: [PackedEval; 6] = [s!(0, 0), s!(11, 2), s!(10, 12), s!(7, 2), s!(4, 5), s!(0, 0)];
pub const MOBILITY_OUTER: [PackedEval; 6] = [s!(0, 0), s!(3, 0), s!(2, 0), s!(3, 1), s!(2, 0), s!(0, 0)];
pub const DOUBLED_PAWN: [PackedEval; 8] = [s!(-1, -8), s!(-15, -20), s!(-25, -41), s!(-54, -49), s!(-22, -39), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ISOLATED_PAWN: [PackedEval; 8] = [s!(-1, -9), s!(-13, -20), s!(-24, -31), s!(-33, -39), s!(-37, -56), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const CHAINED_PAWN: [PackedEval; 8] = [s!(0, 1), s!(10, 8), s!(19, 18), s!(26, 32), s!(32, 49), s!(40, 62), s!(44, 42), s!(0, 0)];
pub const PASSED_PAWN: [PackedEval; 8] = [s!(11, -33), s!(16, 17), s!(19, 55), s!(30, 59), s!(41, 50), s!(22, 41), s!(25, 14), s!(37, 26)];
pub const BACKWARD_PAWN_OPEN_FILE: [PackedEval; 8] = [s!(36, 21), s!(23, 11), s!(10, 0), s!(-6, 8), s!(15, 43), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const BACKWARD_PAWN_CLOSED_FILE: [PackedEval; 8] = [s!(21, 19), s!(17, 16), s!(16, 8), s!(14, 5), s!(14, 41), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD: [PackedEval; 8] = [s!(3, 15), s!(14, 19), s!(20, 20), s!(20, 19), s!(65, 12), s!(20, 21), s!(0, 0), s!(0, 0)];
pub const PAWN_SHIELD_OPEN_FILE: [PackedEval; 8] = [s!(-25, -21), s!(-24, -18), s!(-26, -20), s!(-29, -27), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const KING_AREA_THREATS: [PackedEval; 8] = [s!(-53, 42), s!(-55, 37), s!(-48, 36), s!(-32, 33), s!(0, 16), s!(41, 1), s!(86, -20), s!(166, -56)];
pub const KNIGHT_SAFE_CHECKS: [PackedEval; 8] = [s!(-118, 50), s!(-75, 46), s!(-34, 37), s!(-40, 35), s!(7, 14), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const BISHOP_SAFE_CHECKS: [PackedEval; 8] = [s!(-88, 37), s!(-85, 54), s!(-54, 46), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const ROOK_SAFE_CHECKS: [PackedEval; 8] = [s!(-118, 41), s!(-70, 32), s!(-48, 30), s!(-32, 57), s!(17, 34), s!(0, 0), s!(0, 0), s!(0, 0)];
pub const QUEEN_SAFE_CHECKS: [PackedEval; 8] = [s!(-110, 29), s!(-104, 62), s!(-81, 60), s!(-39, 37), s!(34, 5), s!(82, -5), s!(131, -20), s!(202, -69)];
