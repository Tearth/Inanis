use crate::helpers::*;

struct MagicField {
    pub mask: u64,
    pub shift: i32,
    pub magic_number: u64,
    pub attacks: Box<[u64]>,
}

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

    let x = 10;
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
