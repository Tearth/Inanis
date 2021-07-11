use crate::movescan::Move;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White = 0,
    Black = 1,
}

pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

pub struct Bitboard {
    pub pieces: [[u64; 6]; 2],
    pub occupancy: [u64; 2],
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard {
            pieces: [
                [
                    0x000000000000ff00,
                    0x0000000000000042,
                    0x0000000000000024,
                    0x0000000000000081,
                    0x0000000000000010,
                    0x0000000000000008,
                ],
                [
                    0x00ff000000000000,
                    0x4200000000000000,
                    0x2400000000000000,
                    0x8100000000000000,
                    0x1000000000000000,
                    0x0800000000000000,
                ],
            ],
            occupancy: [0xffff, 0xffff000000000000],
        }
    }

    pub fn get_moves(moves: &[Move]) {}
}
