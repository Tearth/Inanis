use super::bit::*;
use super::representation::CastlingRights;

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
