use super::representation::Board;
use super::representation::CastlingRights;
use super::*;
use crate::utils::assert_fast;
use crate::utils::bitflags::BitFlags;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::rand;

pub const PIECE_HASHES: [[[u64; 64]; 6]; 2] = generate_piece_hashes();
pub const CASTLING_HASHES: [u64; 4] = generate_castling_hashes();
pub const EN_PASSANT_HASHES: [u64; 8] = generate_en_passant_hashes();
pub const ACTIVE_COLOR_HASH: u64 = generate_active_color_hash();

pub const fn generate_piece_hashes() -> [[[u64; 64]; 6]; 2] {
    let mut result = [[[0; 64]; 6]; 2];
    let mut seed = 584578;

    let mut color = 0;
    while color < 2 {
        let mut piece = 0;
        while piece < 6 {
            let mut square = 0;
            while square < 64 {
                let (value, new_seed) = rand::rand(seed);
                result[color][piece][square] = value;
                seed = new_seed;
                square += 1;
            }

            piece += 1;
        }

        color += 1;
    }

    result
}

pub const fn generate_castling_hashes() -> [u64; 4] {
    let mut result = [0; 4];
    let mut seed = 8652221015076841656;

    let mut castling_index = 0;
    while castling_index < 4 {
        let (value, new_seed) = rand::rand(seed);
        result[castling_index] = value;
        seed = new_seed;
        castling_index += 1;
    }

    result
}

pub const fn generate_en_passant_hashes() -> [u64; 8] {
    let mut result = [0; 8];
    let mut seed = 13494315632332173397;

    let mut en_passant_index = 0;
    while en_passant_index < 8 {
        let (value, new_seed) = rand::rand(seed);
        result[en_passant_index] = value;
        seed = new_seed;
        en_passant_index += 1;
    }

    result
}

pub const fn generate_active_color_hash() -> u64 {
    let seed = 13914115299070061278;
    let (value, _) = rand::rand(seed);
    value
}

/// Gets `piece` hash with the `color` for the square specified by `square`.
pub fn get_piece_hash(color: usize, piece: usize, square: usize) -> u64 {
    assert_fast!(color < 2);
    assert_fast!(piece < 6);
    assert_fast!(square < 64);

    PIECE_HASHES[color][piece][square]
}

/// Gets castling right hash based on the `current` ones and the desired change specified by `right`.
pub fn get_castling_right_hash(current: u8, right: u8) -> u64 {
    if !current.contains(right) {
        return 0;
    }

    CASTLING_HASHES[right.bit_scan()]
}

/// Gets en passant hash for the `file`.
pub fn get_en_passant_hash(file: usize) -> u64 {
    assert_fast!(file < 8);
    EN_PASSANT_HASHES[file]
}

/// Gets active color hash.
pub fn get_active_color_hash() -> u64 {
    ACTIVE_COLOR_HASH
}

/// Recalculates board's hash entirely.
pub fn recalculate_hash(board: &mut Board) {
    let mut hash = 0u64;

    for color in ALL_COLORS {
        for piece_index in ALL_PIECES {
            let mut pieces_bb = board.pieces[color][piece_index];
            while pieces_bb != 0 {
                let square_bb = pieces_bb.get_lsb();
                let square = square_bb.bit_scan();
                pieces_bb = pieces_bb.pop_lsb();

                hash ^= zobrist::get_piece_hash(color, piece_index, square);
            }
        }
    }

    if board.state.castling_rights.contains(CastlingRights::WHITE_SHORT_CASTLING) {
        hash ^= zobrist::get_castling_right_hash(board.state.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
    }
    if board.state.castling_rights.contains(CastlingRights::WHITE_LONG_CASTLING) {
        hash ^= zobrist::get_castling_right_hash(board.state.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
    }
    if board.state.castling_rights.contains(CastlingRights::BLACK_SHORT_CASTLING) {
        hash ^= zobrist::get_castling_right_hash(board.state.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
    }
    if board.state.castling_rights.contains(CastlingRights::BLACK_LONG_CASTLING) {
        hash ^= zobrist::get_castling_right_hash(board.state.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
    }

    if board.state.en_passant != 0 {
        hash ^= zobrist::get_en_passant_hash(board.state.en_passant.bit_scan() & 7);
    }

    if board.active_color == BLACK {
        hash ^= zobrist::get_active_color_hash();
    }

    board.state.hash = hash;
}

/// Recalculates board's pawn hash entirely.
pub fn recalculate_pawn_hash(board: &mut Board) {
    let mut hash = 0u64;

    for color in ALL_COLORS {
        for piece in [PAWN, KING] {
            let mut pieces_bb = board.pieces[color][piece];
            while pieces_bb != 0 {
                let square_bb = pieces_bb.get_lsb();
                let square = square_bb.bit_scan();
                pieces_bb = pieces_bb.pop_lsb();

                hash ^= zobrist::get_piece_hash(color, piece, square);
            }
        }
    }

    board.state.pawn_hash = hash;
}
