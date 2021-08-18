use super::board::Bitboard;
use super::board::CastlingRights;
use super::*;

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

                hash ^= get_piece_hash(color, piece_index, field_index);
            }
        }
    }

    if board.castling_rights.contains(CastlingRights::WHITE_SHORT_CASTLING) {
        hash ^= get_castling_right_hash(board.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
    }
    if board.castling_rights.contains(CastlingRights::WHITE_LONG_CASTLING) {
        hash ^= get_castling_right_hash(board.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
    }
    if board.castling_rights.contains(CastlingRights::BLACK_SHORT_CASTLING) {
        hash ^= get_castling_right_hash(board.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
    }
    if board.castling_rights.contains(CastlingRights::BLACK_LONG_CASTLING) {
        hash ^= get_castling_right_hash(board.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
    }

    if board.en_passant != 0 {
        hash ^= get_en_passant_hash((bit_scan(board.en_passant) % 8) as u8);
    }

    if board.active_color == BLACK {
        hash ^= get_active_color_hash();
    }

    board.hash = hash;
}

pub fn get_piece_hash(color: u8, piece: u8, field_index: u8) -> u64 {
    unsafe { PIECE_HASHES[color as usize][piece as usize][field_index as usize] }
}

pub fn get_castling_right_hash(current: CastlingRights, right: CastlingRights) -> u64 {
    if !current.contains(right) {
        return 0;
    }

    unsafe { CASTLING_HASHES[bit_scan(right.bits() as u64) as usize] }
}

pub fn get_en_passant_hash(file: u8) -> u64 {
    unsafe { EN_PASSANT_HASHES[file as usize] }
}

pub fn get_active_color_hash() -> u64 {
    unsafe { ACTIVE_COLOR_HASH }
}
