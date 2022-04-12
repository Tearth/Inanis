// --------------------------------------------------- //
// Generated at 2022-04-11 09:30:33 UTC (e = 0.064058) //
// --------------------------------------------------- //

pub static mut PIECE_VALUE: [i16; 6] = [100, 419, 442, 648, 1325, 10000];

pub static mut PIECE_MOBILITY_OPENING: [i16; 6] = [5, 8, 5, 6, 2, 6];
pub static mut PIECE_MOBILITY_ENDING: [i16; 6] = [3, 0, 0, 5, 6, 3];
pub static mut PIECE_MOBILITY_CENTER_MULTIPLIER: [i16; 6] = [6, 1, 3, 1, 1, 6];

pub static mut DOUBLED_PAWN_OPENING: i16 = 5;
pub static mut DOUBLED_PAWN_ENDING: i16 = -12;

pub static mut ISOLATED_PAWN_OPENING: i16 = -31;
pub static mut ISOLATED_PAWN_ENDING: i16 = 1;

pub static mut CHAINED_PAWN_OPENING: i16 = 5;
pub static mut CHAINED_PAWN_ENDING: i16 = 11;

pub static mut PASSING_PAWN_OPENING: i16 = -1;
pub static mut PASSING_PAWN_ENDING: i16 = 57;

pub static mut PAWN_SHIELD_OPENING: i16 = 12;
pub static mut PAWN_SHIELD_ENDING: i16 = 5;

pub static mut PAWN_SHIELD_OPEN_FILE_OPENING: i16 = -27;
pub static mut PAWN_SHIELD_OPEN_FILE_ENDING: i16 = 2;

pub static mut KING_ATTACKED_FIELDS_OPENING: i16 = -19;
pub static mut KING_ATTACKED_FIELDS_ENDING: i16 = 6;
