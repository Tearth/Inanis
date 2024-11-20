use std::ops::RangeInclusive;

pub mod movegen;
pub mod movescan;
pub mod patterns;
pub mod representation;
pub mod text;
pub mod zobrist;

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub const US: usize = 0;
pub const THEM: usize = 1;

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

pub const CENTER_BB: u64 = 0x3c3c3c3c0000;
pub const OUTSIDE_BB: u64 = 0xffffc3c3c3c3ffff;
pub const EDGE_BB: u64 = 0xff818181818181ff;

pub const WHITE_SQUARES_BB: u64 = 0xaa55aa55aa55aa55;
pub const BLACK_SQUARES_BB: u64 = 0x55aa55aa55aa55aa;

pub const FILE_A_BB: u64 = 0x0101010101010101 << 7;
pub const FILE_B_BB: u64 = 0x0101010101010101 << 6;
pub const FILE_C_BB: u64 = 0x0101010101010101 << 5;
pub const FILE_D_BB: u64 = 0x0101010101010101 << 4;
pub const FILE_E_BB: u64 = 0x0101010101010101 << 3;
pub const FILE_F_BB: u64 = 0x0101010101010101 << 2;
pub const FILE_G_BB: u64 = 0x0101010101010101 << 1;
pub const FILE_H_BB: u64 = 0x0101010101010101 << 0;

pub const RANK_1_BB: u64 = 0x00000000000000ff << 0;
pub const RANK_2_BB: u64 = 0x00000000000000ff << 8;
pub const RANK_3_BB: u64 = 0x00000000000000ff << 16;
pub const RANK_4_BB: u64 = 0x00000000000000ff << 24;
pub const RANK_5_BB: u64 = 0x00000000000000ff << 32;
pub const RANK_6_BB: u64 = 0x00000000000000ff << 40;
pub const RANK_7_BB: u64 = 0x00000000000000ff << 48;
pub const RANK_8_BB: u64 = 0x00000000000000ff << 56;

pub const A1: usize = 7;
pub const B1: usize = 6;
pub const C1: usize = 5;
pub const D1: usize = 4;
pub const E1: usize = 3;
pub const F1: usize = 2;
pub const G1: usize = 1;
pub const H1: usize = 0;
pub const A1_BB: u64 = 1 << A1;
pub const B1_BB: u64 = 1 << B1;
pub const C1_BB: u64 = 1 << C1;
pub const D1_BB: u64 = 1 << D1;
pub const E1_BB: u64 = 1 << E1;
pub const F1_BB: u64 = 1 << F1;
pub const G1_BB: u64 = 1 << G1;
pub const H1_BB: u64 = 1 << H1;

pub const A2: usize = 15;
pub const B2: usize = 14;
pub const C2: usize = 13;
pub const D2: usize = 12;
pub const E2: usize = 11;
pub const F2: usize = 10;
pub const G2: usize = 9;
pub const H2: usize = 8;
pub const A2_BB: u64 = 1 << A2;
pub const B2_BB: u64 = 1 << B2;
pub const C2_BB: u64 = 1 << C2;
pub const D2_BB: u64 = 1 << D2;
pub const E2_BB: u64 = 1 << E2;
pub const F2_BB: u64 = 1 << F2;
pub const G2_BB: u64 = 1 << G2;
pub const H2_BB: u64 = 1 << H2;

pub const A3: usize = 23;
pub const B3: usize = 22;
pub const C3: usize = 21;
pub const D3: usize = 20;
pub const E3: usize = 19;
pub const F3: usize = 18;
pub const G3: usize = 17;
pub const H3: usize = 16;
pub const A3_BB: u64 = 1 << A3;
pub const B3_BB: u64 = 1 << B3;
pub const C3_BB: u64 = 1 << C3;
pub const D3_BB: u64 = 1 << D3;
pub const E3_BB: u64 = 1 << E3;
pub const F3_BB: u64 = 1 << F3;
pub const G3_BB: u64 = 1 << G3;
pub const H3_BB: u64 = 1 << H3;

pub const A4: usize = 31;
pub const B4: usize = 30;
pub const C4: usize = 29;
pub const D4: usize = 28;
pub const E4: usize = 27;
pub const F4: usize = 26;
pub const G4: usize = 25;
pub const H4: usize = 24;
pub const A4_BB: u64 = 1 << A4;
pub const B4_BB: u64 = 1 << B4;
pub const C4_BB: u64 = 1 << C4;
pub const D4_BB: u64 = 1 << D4;
pub const E4_BB: u64 = 1 << E4;
pub const F4_BB: u64 = 1 << F4;
pub const G4_BB: u64 = 1 << G4;
pub const H4_BB: u64 = 1 << H4;

pub const A5: usize = 39;
pub const B5: usize = 38;
pub const C5: usize = 37;
pub const D5: usize = 36;
pub const E5: usize = 35;
pub const F5: usize = 34;
pub const G5: usize = 33;
pub const H5: usize = 32;
pub const A5_BB: u64 = 1 << A5;
pub const B5_BB: u64 = 1 << B5;
pub const C5_BB: u64 = 1 << C5;
pub const D5_BB: u64 = 1 << D5;
pub const E5_BB: u64 = 1 << E5;
pub const F5_BB: u64 = 1 << F5;
pub const G5_BB: u64 = 1 << G5;
pub const H5_BB: u64 = 1 << H5;

pub const A6: usize = 47;
pub const B6: usize = 46;
pub const C6: usize = 45;
pub const D6: usize = 44;
pub const E6: usize = 43;
pub const F6: usize = 42;
pub const G6: usize = 41;
pub const H6: usize = 40;
pub const A6_BB: u64 = 1 << A6;
pub const B6_BB: u64 = 1 << B6;
pub const C6_BB: u64 = 1 << C6;
pub const D6_BB: u64 = 1 << D6;
pub const E6_BB: u64 = 1 << E6;
pub const F6_BB: u64 = 1 << F6;
pub const G6_BB: u64 = 1 << G6;
pub const H6_BB: u64 = 1 << H6;

pub const A7: usize = 55;
pub const B7: usize = 54;
pub const C7: usize = 53;
pub const D7: usize = 52;
pub const E7: usize = 51;
pub const F7: usize = 50;
pub const G7: usize = 49;
pub const H7: usize = 48;
pub const A7_BB: u64 = 1 << A7;
pub const B7_BB: u64 = 1 << B7;
pub const C7_BB: u64 = 1 << C7;
pub const D7_BB: u64 = 1 << D7;
pub const E7_BB: u64 = 1 << E7;
pub const F7_BB: u64 = 1 << F7;
pub const G7_BB: u64 = 1 << G7;
pub const H7_BB: u64 = 1 << H7;

pub const A8: usize = 63;
pub const B8: usize = 62;
pub const C8: usize = 61;
pub const D8: usize = 60;
pub const E8: usize = 59;
pub const F8: usize = 58;
pub const G8: usize = 57;
pub const H8: usize = 56;
pub const A8_BB: u64 = 1 << A8;
pub const B8_BB: u64 = 1 << B8;
pub const C8_BB: u64 = 1 << C8;
pub const D8_BB: u64 = 1 << D8;
pub const E8_BB: u64 = 1 << E8;
pub const F8_BB: u64 = 1 << F8;
pub const G8_BB: u64 = 1 << G8;
pub const H8_BB: u64 = 1 << H8;

pub const ALL_COLORS: RangeInclusive<usize> = 0..=1;
pub const ALL_POVS: RangeInclusive<usize> = 0..=1;
pub const ALL_PHASES: RangeInclusive<usize> = 0..=1;
pub const ALL_PIECES: RangeInclusive<usize> = 0..=5;
pub const ALL_FILES: RangeInclusive<usize> = 0..=7;
pub const ALL_RANKS: RangeInclusive<usize> = 0..=7;
pub const ALL_SQUARES: RangeInclusive<usize> = 0..=63;
