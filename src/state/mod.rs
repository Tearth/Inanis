pub mod board;
pub mod fen;
pub mod movegen;
pub mod movescan;
pub mod patterns;
pub mod zobrist;

pub const WHITE: u8 = 0;
pub const BLACK: u8 = 1;

pub const OPENING: u8 = 0;
pub const ENDING: u8 = 1;

pub const PAWN: u8 = 0;
pub const KNIGHT: u8 = 1;
pub const BISHOP: u8 = 2;
pub const ROOK: u8 = 3;
pub const QUEEN: u8 = 4;
pub const KING: u8 = 5;

pub const LEFT: u8 = 0;
pub const RIGHT: u8 = 1;

pub const FILE_A: u64 = 0x8080808080808080;
pub const FILE_B: u64 = 0x4040404040404040;
pub const FILE_C: u64 = 0x2020202020202020;
pub const FILE_D: u64 = 0x1010101010101010;
pub const FILE_E: u64 = 0x0808080808080808;
pub const FILE_F: u64 = 0x0404040404040404;
pub const FILE_G: u64 = 0x0202020202020202;
pub const FILE_H: u64 = 0x0101010101010101;

pub const RANK_A: u64 = 0x00000000000000FF;
pub const RANK_B: u64 = 0x000000000000FF00;
pub const RANK_C: u64 = 0x0000000000FF0000;
pub const RANK_D: u64 = 0x00000000FF000000;
pub const RANK_E: u64 = 0x000000FF00000000;
pub const RANK_F: u64 = 0x0000FF0000000000;
pub const RANK_G: u64 = 0x00FF000000000000;
pub const RANK_H: u64 = 0xFF00000000000000;

pub const CENTER: u64 = 0x3c3c3c3c0000;
pub const OUTSIDE: u64 = 0xffffc3c3c3c3ffff;
pub const EDGE: u64 = 0xff818181818181ff;

pub const WHITE_FIELDS: u64 = 0xaa55aa55aa55aa55;
pub const BLACK_FIELDS: u64 = 0x55aa55aa55aa55aa;

/// Extracts the lowest set isolated bit.
///
/// More about asm instruction: <https://www.felixcloutier.com/x86/blsi>
#[inline(always)]
pub fn get_lsb(value: u64) -> u64 {
    value & value.wrapping_neg()
}

/// Resets the lowest set bit.
///
/// More about asm instruction: <https://www.felixcloutier.com/x86/blsr>
#[inline(always)]
pub fn pop_lsb(value: u64) -> u64 {
    value & (value - 1)
}

/// Counts the number of set bits.
///
/// More about asm instruction: <https://www.felixcloutier.com/x86/popcnt>
#[inline(always)]
pub fn bit_count(value: u64) -> u8 {
    value.count_ones() as u8
}

/// Gets an index of the first set bit by counting trailing zero bits.
///
/// More about asm instruction: <https://www.felixcloutier.com/x86/tzcnt>
#[inline(always)]
pub fn bit_scan(value: u64) -> u8 {
    value.trailing_zeros() as u8
}

/// Converts piece `symbol` (p/P, n/N, b/B, r/R, q/Q, k/K) into the corresponding [u8] value. Returns [Err] with the proper error messages when the `symbol` is unknown.
pub fn symbol_to_piece(symbol: char) -> Result<u8, &'static str> {
    match symbol {
        'p' | 'P' => Ok(PAWN),
        'n' | 'N' => Ok(KNIGHT),
        'b' | 'B' => Ok(BISHOP),
        'r' | 'R' => Ok(ROOK),
        'q' | 'Q' => Ok(QUEEN),
        'k' | 'K' => Ok(KING),
        _ => Err("Invalid FEN: bad piece symbol"),
    }
}

/// Converts `piece` into the corresponding character (p/P, n/N, b/B, r/R, q/Q, k/K). Returns [Err] with the proper error message when the `piece` is unknown.
pub fn piece_to_symbol(piece: u8) -> Result<char, &'static str> {
    match piece {
        PAWN => Ok('P'),
        KNIGHT => Ok('N'),
        BISHOP => Ok('B'),
        ROOK => Ok('R'),
        QUEEN => Ok('Q'),
        KING => Ok('K'),
        _ => Err("Invalid piece symbol"),
    }
}
