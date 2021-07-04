use crate::constants::*;
use crate::helpers::*;
use crate::patterns::*;
use arr_macro::arr;

#[rustfmt::skip]
static MAGIC_ROOK_SHIFTS: [i32; 64] =
[
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12
];

#[rustfmt::skip]
static MAGIC_BISHOP_SHIFTS: [i32; 64] =
[
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

struct MagicField {
    pub mask: u64,
    pub shift: i32,
    pub magic_number: u64,
    pub attacks: Vec<u64>,
}

impl MagicField {
    pub const fn new() -> MagicField {
        MagicField {
            mask: 0,
            shift: 0,
            magic_number: 0,
            attacks: Vec::new(),
        }
    }
}

static mut MAGIC_ROOK_FIELDS: [MagicField; 64] = arr!(MagicField::new(); 64);
static mut MAGIC_BISHOP_FIELDS: [MagicField; 64] = arr!(MagicField::new(); 64);

pub fn magic_init() {
    let _test01 = magic_get_permutation(0x1010106e101000, 0);
    let _test02 = magic_get_permutation(0x1010106e101000, 1);
    let _test03 = magic_get_permutation(0x1010106e101000, 2);
    let _test04 = magic_get_permutation(0x1010106e101000, 3);
    let _test05 = magic_get_permutation(0x1010106e101000, 4);
    let _test06 = magic_get_permutation(0x1010106e101000, 5);
    let _test07 = magic_get_permutation(0x1010106e101000, 6);

    let _test08 = magic_get_rook_attacks(0x10300001001000, 28);
    let _test09 = magic_get_bishop_attacks(0x10300001001000, 28);
    let _test10 = magic_get_rook_attacks(0x10300001001000, 0);
    let _test11 = magic_get_bishop_attacks(0x10300001001000, 0);

    for index in 0..64 {
        println!("{}", magic_generate_rook_number_for_field(index));
    }

    for index in 0..64 {
        println!("{}", magic_generate_bishop_number_for_field(index));
    }

    loop {}
}

fn magic_generate_rook_number_for_field(field_index: i32) -> u64 {
    let shift = MAGIC_ROOK_SHIFTS[field_index as usize];
    let count = 1 << shift;
    let mask = magic_get_rook_mask(field_index);

    let mut permutations = Vec::with_capacity(count as usize);
    let mut attacks = Vec::with_capacity(count as usize);

    for index in 0..count {
        let permutation = magic_get_permutation(mask, index as u64);

        permutations.push(permutation);
        attacks.push(magic_get_rook_attacks(permutation, field_index));
    }

    magic_generate_number(shift, count, &permutations, &attacks)
}

fn magic_generate_bishop_number_for_field(field_index: i32) -> u64 {
    let shift = MAGIC_BISHOP_SHIFTS[field_index as usize];
    let count = 1 << shift;
    let mask = magic_get_bishop_mask(field_index);

    let mut permutations = Vec::with_capacity(count as usize);
    let mut attacks = Vec::with_capacity(count as usize);

    for index in 0..count {
        let permutation = magic_get_permutation(mask, index as u64);

        permutations.push(permutation);
        attacks.push(magic_get_bishop_attacks(permutation, field_index));
    }

    magic_generate_number(shift, count, &permutations, &attacks)
}

fn magic_generate_number(shift: i32, count: i32, permutations: &Vec<u64>, attacks: &Vec<u64>) -> u64 {
    let mut final_attacks = Vec::with_capacity(count as usize);
    final_attacks.resize(count as usize, 0);

    let mut found = false;
    let mut magic_number = 0u64;

    while !found {
        found = true;
        magic_number = fastrand::u64(1..u64::MAX) & fastrand::u64(1..u64::MAX) & fastrand::u64(1..u64::MAX);

        for index in 0..count {
            let hash = (permutations[index as usize].wrapping_mul(magic_number)) >> (64 - shift);

            if final_attacks[hash as usize] == 0 || final_attacks[hash as usize] == attacks[index as usize] {
                final_attacks[hash as usize] = attacks[index as usize];
            } else {
                found = false;
                break;
            }
        }

        if found {
            break;
        }

        for index in &mut final_attacks {
            *index = 0;
        }
    }

    magic_number
}

fn magic_get_permutation(mut mask: u64, mut index: u64) -> u64 {
    let mut result = 0u64;

    while mask != 0 {
        let lsb = get_lsb(mask);
        let lsb_index = bit_scan(lsb);
        mask = pop_lsb(mask);

        result |= (index & 1) << lsb_index;
        index >>= 1;
    }

    result
}

fn magic_get_rook_mask(field_index: i32) -> u64 {
    (patterns_get_file(field_index) & !RANK_A & !RANK_H) | (patterns_get_rank(field_index) & !FILE_A & !FILE_H)
}

fn magic_get_bishop_mask(field_index: i32) -> u64 {
    patterns_get_diagonals(field_index) & !EDGE
}

fn magic_get_rook_attacks(bitboard: u64, field_index: i32) -> u64 {
    let result = 0
        | magic_get_attacks(bitboard, field_index, (1, 0))
        | magic_get_attacks(bitboard, field_index, (-1, 0))
        | magic_get_attacks(bitboard, field_index, (0, 1))
        | magic_get_attacks(bitboard, field_index, (0, -1));

    result
}

fn magic_get_bishop_attacks(bitboard: u64, field_index: i32) -> u64 {
    let result = 0
        | magic_get_attacks(bitboard, field_index, (1, 1))
        | magic_get_attacks(bitboard, field_index, (-1, 1))
        | magic_get_attacks(bitboard, field_index, (1, -1))
        | magic_get_attacks(bitboard, field_index, (-1, -1));

    result
}

fn magic_get_attacks(bitboard: u64, field_index: i32, direction: (i32, i32)) -> u64 {
    let mut result = 0u64;
    let mut current = (field_index % 8 + direction.0, field_index / 8 + direction.1);

    while current.0 >= 0 && current.0 <= 7 && current.1 >= 0 && current.1 <= 7 {
        result |= 1u64 << (current.0 + current.1 * 8);

        if (bitboard & result) != 0 {
            break;
        }

        current = (current.0 + direction.0, current.1 + direction.1);
    }

    result
}
