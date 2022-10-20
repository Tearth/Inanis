use super::representation::Board;
use super::representation::CastlingRights;
use super::*;
use crate::utils::bitflags::BitFlags;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::rand;

pub struct ZobristContainer {
    piece_hashes: [[[u64; 64]; 6]; 2],
    castling_hashes: [u64; 4],
    en_passant_hashes: [u64; 8],
    active_color_hash: u64,
}

impl ZobristContainer {
    /// Gets `piece` hash with the `color` for the square specified by `square`.
    pub fn get_piece_hash(&self, color: usize, piece: usize, square: usize) -> u64 {
        self.piece_hashes[color][piece][square]
    }

    /// Gets castling right hash based on the `current` ones and the desired change specified by `right`.
    pub fn get_castling_right_hash(&self, current: u8, right: u8) -> u64 {
        if !current.contains(right) {
            return 0;
        }

        self.castling_hashes[right.bit_scan()]
    }

    /// Gets en passant hash for the `file`.
    pub fn get_en_passant_hash(&self, file: usize) -> u64 {
        self.en_passant_hashes[file]
    }

    /// Gets active color hash.
    pub fn get_active_color_hash(&self) -> u64 {
        self.active_color_hash
    }
}

impl Default for ZobristContainer {
    /// Constructs a default instance of [ZobristContainer] with initialized hashes.
    fn default() -> Self {
        let mut result = Self { piece_hashes: [[[0; 64]; 6]; 2], castling_hashes: [0; 4], en_passant_hashes: [0; 8], active_color_hash: 0 };

        rand::seed(584578);

        for color in ALL_COLORS {
            for piece in ALL_PIECES {
                for square in ALL_SQUARES {
                    result.piece_hashes[color][piece][square] = rand::u64(..);
                }
            }
        }

        for castling_index in 0..4 {
            result.castling_hashes[castling_index] = rand::u64(..);
        }

        for en_passant_index in ALL_FILES {
            result.en_passant_hashes[en_passant_index] = rand::u64(..);
        }

        result.active_color_hash = rand::u64(..);
        result
    }
}

/// Recalculates board's hash entirely.
pub fn recalculate_hash(board: &mut Board) {
    let mut hash = 0u64;

    for color in ALL_COLORS {
        for piece_index in ALL_PIECES {
            let mut pieces = board.pieces[color][piece_index];
            while pieces != 0 {
                let square_bb = pieces.get_lsb();
                let square = square_bb.bit_scan();
                pieces = pieces.pop_lsb();

                hash ^= board.zobrist.get_piece_hash(color, piece_index, square);
            }
        }
    }

    if board.castling_rights.contains(CastlingRights::WHITE_SHORT_CASTLING) {
        hash ^= board.zobrist.get_castling_right_hash(board.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
    }
    if board.castling_rights.contains(CastlingRights::WHITE_LONG_CASTLING) {
        hash ^= board.zobrist.get_castling_right_hash(board.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
    }
    if board.castling_rights.contains(CastlingRights::BLACK_SHORT_CASTLING) {
        hash ^= board.zobrist.get_castling_right_hash(board.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
    }
    if board.castling_rights.contains(CastlingRights::BLACK_LONG_CASTLING) {
        hash ^= board.zobrist.get_castling_right_hash(board.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
    }

    if board.en_passant != 0 {
        hash ^= board.zobrist.get_en_passant_hash(board.en_passant.bit_scan() & 7);
    }

    if board.active_color == BLACK {
        hash ^= board.zobrist.get_active_color_hash();
    }

    board.hash = hash;
}

/// Recalculates board's pawn hash entirely.
pub fn recalculate_pawn_hash(board: &mut Board) {
    let mut hash = 0u64;

    for color in ALL_COLORS {
        for piece in [PAWN, KING] {
            let mut pieces = board.pieces[color][piece];
            while pieces != 0 {
                let square_bb = pieces.get_lsb();
                let square = square_bb.bit_scan();
                pieces = pieces.pop_lsb();

                hash ^= board.zobrist.get_piece_hash(color, piece, square);
            }
        }
    }

    board.pawn_hash = hash;
}
