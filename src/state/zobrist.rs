use super::*;
use crate::utils::bitflags::BitFlags;
use crate::utils::rand;

pub struct ZobristContainer {
    piece_hashes: [[[u64; 64]; 6]; 2],
    castling_hashes: [u64; 4],
    en_passant_hashes: [u64; 8],
    active_color_hash: u64,
}

impl ZobristContainer {
    /// Gets `piece` hash with the `color` for the square specified by `square_index`.
    pub fn get_piece_hash(&self, color: u8, piece: u8, square_index: u8) -> u64 {
        self.piece_hashes[color as usize][piece as usize][square_index as usize]
    }

    /// Gets castling right hash based on the `current` ones and the desired change specified by `right`.
    pub fn get_castling_right_hash(&self, current: u8, right: u8) -> u64 {
        if !current.contains(right) {
            return 0;
        }

        self.castling_hashes[bit_scan(right as u64) as usize]
    }

    /// Gets en passant hash for the `file`.
    pub fn get_en_passant_hash(&self, file: u8) -> u64 {
        self.en_passant_hashes[file as usize]
    }

    /// Gets active color hash.
    pub fn get_active_color_hash(&self) -> u64 {
        self.active_color_hash
    }
}

impl Default for ZobristContainer {
    /// Constructs a default instance of [ZobristContainer] with initialized hashes.
    fn default() -> Self {
        let mut result = Self {
            piece_hashes: [[[0; 64]; 6]; 2],
            castling_hashes: [0; 4],
            en_passant_hashes: [0; 8],
            active_color_hash: 0,
        };

        rand::seed(584578);

        for color in 0..2 {
            for piece in 0..6 {
                for square_index in 0..64 {
                    result.piece_hashes[color as usize][piece as usize][square_index] = rand::u64(..);
                }
            }
        }

        for castling_index in 0..4 {
            result.castling_hashes[castling_index as usize] = rand::u64(..);
        }

        for en_passant_index in 0..8 {
            result.en_passant_hashes[en_passant_index as usize] = rand::u64(..);
        }

        result.active_color_hash = rand::u64(..);
        result
    }
}
