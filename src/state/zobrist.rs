use super::board::{Bitboard, CastlingRights};
use super::common::*;

static mut PIECE_HASHES: [[[u64; 64]; 6]; 2] = [[[0; 64]; 6]; 2];
static mut CASTLING_HASHES: [u64; 4] = [0; 4];
static mut EN_PASSANT_HASHES: [u64; 8] = [0; 8];
static mut ACTIVE_COLOR_HASH: u64 = 0;

pub fn init() {
    unsafe {
        for color_hash in PIECE_HASHES.iter_mut() {
            for piece_hash in color_hash.iter_mut() {
                for field_hash in piece_hash.iter_mut() {
                    *field_hash = fastrand::u64(1..u64::MAX);
                }
            }
        }

        for castling_hash in CASTLING_HASHES.iter_mut() {
            *castling_hash = fastrand::u64(1..u64::MAX);
        }

        for en_passant_hash in EN_PASSANT_HASHES.iter_mut() {
            *en_passant_hash = fastrand::u64(1..u64::MAX);
        }

        ACTIVE_COLOR_HASH = fastrand::u64(1..u64::MAX);
    }
}

pub fn recalculate_hash(board: &mut Bitboard) {
    let mut hash = 0u64;

    for color in 0..2 {
        for piece_index in 0..6 {
            let mut pieces = board.pieces[color as usize][piece_index as usize];
            while pieces != 0 {
                let field = get_lsb(pieces);
                let field_index = bit_scan(field);
                pieces = pop_lsb(pieces);

                toggle_piece(&mut hash, color, piece_index, field_index);
            }
        }
    }

    if board.castling_rights.contains(CastlingRights::WHITE_SHORT_CASTLING) {
        toggle_castling_right(&mut hash, board.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
    }
    if board.castling_rights.contains(CastlingRights::WHITE_LONG_CASTLING) {
        toggle_castling_right(&mut hash, board.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
    }
    if board.castling_rights.contains(CastlingRights::BLACK_SHORT_CASTLING) {
        toggle_castling_right(&mut hash, board.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
    }
    if board.castling_rights.contains(CastlingRights::BLACK_LONG_CASTLING) {
        toggle_castling_right(&mut hash, board.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
    }

    if board.en_passant != 0 {
        toggle_en_passant(&mut hash, (bit_scan(board.en_passant) % 8) as u8);
    }

    if board.active_color == BLACK {
        toggle_active_color(&mut hash);
    }

    board.hash = hash;
}

pub fn toggle_piece(hash: &mut u64, color: u8, piece: u8, field_index: u8) {
    *hash ^= unsafe { PIECE_HASHES[color as usize][piece as usize][field_index as usize] };
}

pub fn toggle_castling_right(hash: &mut u64, current: CastlingRights, right: CastlingRights) {
    if current.contains(right) {
        *hash ^= unsafe { CASTLING_HASHES[bit_scan(right.bits() as u64) as usize] };
    }
}

pub fn toggle_en_passant(hash: &mut u64, file: u8) {
    *hash ^= unsafe { EN_PASSANT_HASHES[file as usize] };
}

pub fn toggle_active_color(hash: &mut u64) {
    *hash ^= unsafe { ACTIVE_COLOR_HASH };
}
