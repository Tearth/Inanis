pub mod movegen;
pub mod movescan;
pub mod patterns;
pub mod representation;
pub mod text;
pub mod zobrist;

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub const OPENING: usize = 0;
pub const ENDING: usize = 1;

pub const PAWN: usize = 0;
pub const KNIGHT: usize = 1;
pub const BISHOP: usize = 2;
pub const ROOK: usize = 3;
pub const QUEEN: usize = 4;
pub const KING: usize = 5;

pub const LEFT: usize = 0;
pub const RIGHT: usize = 1;

// Used only in iterators, doesn't relate to the bitboard layout
pub const FILE_A: usize = 0;
pub const FILE_H: usize = 7;

pub const FILE_A_BB: u64 = 0x0101010101010101 << 7;
pub const FILE_B_BB: u64 = 0x0101010101010101 << 6;
pub const FILE_C_BB: u64 = 0x0101010101010101 << 5;
pub const FILE_D_BB: u64 = 0x0101010101010101 << 4;
pub const FILE_E_BB: u64 = 0x0101010101010101 << 3;
pub const FILE_F_BB: u64 = 0x0101010101010101 << 2;
pub const FILE_G_BB: u64 = 0x0101010101010101 << 1;
pub const FILE_H_BB: u64 = 0x0101010101010101 << 0;

// Used only in iterators, doesn't relate to the bitboard layout
pub const RANK_1: usize = 0;
pub const RANK_8: usize = 7;

pub const RANK_1_BB: u64 = 0x00000000000000ff << 0;
pub const RANK_2_BB: u64 = 0x00000000000000ff << 8;
pub const RANK_3_BB: u64 = 0x00000000000000ff << 16;
pub const RANK_4_BB: u64 = 0x00000000000000ff << 24;
pub const RANK_5_BB: u64 = 0x00000000000000ff << 32;
pub const RANK_6_BB: u64 = 0x00000000000000ff << 40;
pub const RANK_7_BB: u64 = 0x00000000000000ff << 48;
pub const RANK_8_BB: u64 = 0x00000000000000ff << 56;

// Used only in iterators, doesn't relate to the bitboard layout
pub const A1: usize = 0;
pub const H8: usize = 63;

pub const CENTER_BB: u64 = 0x3c3c3c3c0000;
pub const OUTSIDE_BB: u64 = 0xffffc3c3c3c3ffff;
pub const EDGE_BB: u64 = 0xff818181818181ff;

pub const WHITE_FIELDS_BB: u64 = 0xaa55aa55aa55aa55;
pub const BLACK_FIELDS_BB: u64 = 0x55aa55aa55aa55aa;
