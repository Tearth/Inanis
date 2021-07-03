use crate::constants::*;

static mut JUMP_PATTERNS: [u64; 64] = [0; 64];
static mut BOX_PATTERNS: [u64; 64] = [0; 64];

pub fn patterns_init() {
    patterns_generate_jumps();
    patterns_generate_boxes();
}

pub fn patterns_get_file(file: i32) -> u64 {
    0x101010101010101 << file
}

pub fn patterns_get_rank(rank: i32) -> u64 {
    0xff << 8 * rank
}

pub fn patterns_get_jump(field_index: usize) -> u64 {
    unsafe { JUMP_PATTERNS[field_index] }
}

pub fn patterns_get_box(field_index: usize) -> u64 {
    unsafe { BOX_PATTERNS[field_index] }
}

fn patterns_generate_jumps() {
    unsafe {
        for field_index in 0..64 {
            let field = 1u64 << field_index;
            JUMP_PATTERNS[field_index] = ((field & !FILE_A) << 17)
                | ((field & !FILE_H) << 15)
                | ((field & !FILE_A & !FILE_B) << 10)
                | ((field & !FILE_G & !FILE_H) << 6)
                | ((field & !FILE_A & !FILE_B) >> 6)
                | ((field & !FILE_G & !FILE_H) >> 10)
                | ((field & !FILE_A) >> 15)
                | ((field & !FILE_H) >> 17);
        }
    }
}

fn patterns_generate_boxes() {
    unsafe {
        for field_index in 0..64 {
            let field = 1u64 << field_index;
            BOX_PATTERNS[field_index] = ((field & !FILE_A) << 1)
                | ((field & !FILE_H) << 7)
                | (field << 8)
                | ((field & !FILE_A) << 9)
                | ((field & !FILE_H) >> 1)
                | ((field & !FILE_A) >> 7)
                | (field >> 8)
                | ((field & !FILE_H) >> 9);
        }
    }
}
