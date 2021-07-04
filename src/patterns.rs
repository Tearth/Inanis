use crate::constants::*;

static mut FILE_PATTERNS: [u64; 64] = [0; 64];
static mut RANK_PATTERNS: [u64; 64] = [0; 64];
static mut DIAGONAL_PATTERNS: [u64; 64] = [0; 64];
static mut JUMP_PATTERNS: [u64; 64] = [0; 64];
static mut BOX_PATTERNS: [u64; 64] = [0; 64];

pub fn patterns_init() {
    patterns_generate_files();
    patterns_generate_ranks();
    patterns_generate_diagonals();
    patterns_generate_jumps();
    patterns_generate_boxes();
}

pub fn patterns_get_file(field_index: i32) -> u64 {
    unsafe { FILE_PATTERNS[field_index as usize] }
}

pub fn patterns_get_rank(field_index: i32) -> u64 {
    unsafe { RANK_PATTERNS[field_index as usize] }
}

pub fn patterns_get_diagonals(field_index: i32) -> u64 {
    unsafe { DIAGONAL_PATTERNS[field_index as usize] }
}

pub fn patterns_get_jumps(field_index: i32) -> u64 {
    unsafe { JUMP_PATTERNS[field_index as usize] }
}

pub fn patterns_get_box(field_index: i32) -> u64 {
    unsafe { BOX_PATTERNS[field_index as usize] }
}

fn patterns_generate_files() {
    unsafe {
        for field_index in 0..64 {
            FILE_PATTERNS[field_index as usize] = (0x101010101010101 << (field_index % 8)) & !(1u64 << field_index);
        }
    }
}
fn patterns_generate_ranks() {
    unsafe {
        for field_index in 0..64 {
            RANK_PATTERNS[field_index as usize] = (0xff << 8 * (field_index / 8)) & !(1u64 << field_index);
        }
    }
}

fn patterns_generate_diagonals() {
    unsafe {
        for field_index in 0..64 {
            let mut result = 0u64;

            for direction in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                let mut current = (field_index % 8 + direction.0, field_index / 8 + direction.1);

                while current.0 >= 0 && current.0 <= 7 && current.1 >= 0 && current.1 <= 7 {
                    result |= 1u64 << (current.0 + current.1 * 8);
                    current = (current.0 + direction.0, current.1 + direction.1);
                }
            }

            DIAGONAL_PATTERNS[field_index as usize] = result;
        }
    }
}

fn patterns_generate_jumps() {
    unsafe {
        for field_index in 0..64 {
            let field = 1u64 << field_index;

            JUMP_PATTERNS[field_index as usize] = 0
                | ((field & !FILE_G & !FILE_H) << 6)
                | ((field & !FILE_A & !FILE_B) >> 6)
                | ((field & !FILE_A & !FILE_B) << 10)
                | ((field & !FILE_G & !FILE_H) >> 10)
                | ((field & !FILE_H) << 15)
                | ((field & !FILE_A) >> 15)
                | ((field & !FILE_A) << 17)
                | ((field & !FILE_H) >> 17);
        }
    }
}

fn patterns_generate_boxes() {
    unsafe {
        for field_index in 0..64 {
            let field = 1u64 << field_index;

            BOX_PATTERNS[field_index as usize] = 0
                | ((field & !FILE_A) << 1)
                | ((field & !FILE_H) >> 1)
                | ((field & !FILE_H) << 7)
                | ((field & !FILE_A) >> 7)
                | ((field & !RANK_H) << 8)
                | ((field & !RANK_A) >> 8)
                | ((field & !FILE_A) << 9)
                | ((field & !FILE_H) >> 9);
        }
    }
}
