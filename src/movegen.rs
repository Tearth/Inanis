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

static MAGIC_ROOK_NUMBERS: [u64; 64] = [
    2413929538783315984,
    1170945936180973568,
    36063983556362632,
    72092815185739788,
    10520444128667370504,
    216173881759761408,
    144117387214979332,
    36035675409096960,
    9147939964463232,
    9241527447739047936,
    4900198006996009024,
    9271504309680349440,
    290904697693279232,
    18577391417034800,
    6557382345249260032,
    576601508053778688,
    289394209213907079,
    9259827994678018049,
    9227875773923722240,
    3458800797972758600,
    5260345653422196736,
    1153485004358042624,
    144401061158326274,
    108088590096941196,
    9835864061152075800,
    1738671485194616848,
    54184079046820352,
    4611703612763537536,
    4917935201873035392,
    72061994232053888,
    72642538535649792,
    144186664922138796,
    9223408595893223562,
    36662253159714817,
    9148830171799556,
    13907258654559569920,
    4644904059815936,
    1153491053785908224,
    37525196376080,
    576742263804133474,
    4611721477685805056,
    5046847994054443074,
    81347780144070704,
    4652218965232287760,
    613641840008167440,
    1208090608632365184,
    1444811064045797444,
    144115480410849301,
    36031270989333568,
    9024809304326464,
    6918690112188612736,
    11681520772186240,
    4612249003011083776,
    1153205180755083392,
    4657900772905853952,
    578724114186912256,
    16648963476391231489,
    504544995536095746,
    4036492191583969290,
    9264080424097810437,
    18577417855566882,
    562984515535106,
    3206563493302044676,
    2323860433544422402,
];

static MAGIC_BISHOP_NUMBERS: [u64; 64] = [
    18018831217729569,
    7566619154406457857,
    5769265629886676992,
    11836590258269718532,
    1130711343955976,
    5188711992789051600,
    2595223508957332480,
    4613977417873621504,
    52914355962521,
    720584771016851496,
    81672282392551681,
    1443689575104709154,
    2315497890614674434,
    4504720752246850,
    4683757923315231234,
    36030472065647684,
    869195003027850256,
    2533343613162048,
    9077585216569856,
    4613938370177974404,
    1819740157378038914,
    3026981903850120192,
    150100551208960,
    9250534398112121088,
    13983151348368280608,
    2453353491453707520,
    1189838711584489508,
    5188437052588310784,
    73192707253608448,
    1171008470900605200,
    2304869587485219,
    72216611327312384,
    1416312765554720,
    4902326593860936194,
    7494553009214460168,
    4535619944704,
    289464028735078528,
    9572356859152384,
    24771999121551488,
    4611968773312806976,
    594765529323978768,
    290309994972256,
    73747544500340740,
    4504287224791296,
    18109643855331840,
    9242583324889711104,
    302884692999082816,
    2315421958013010688,
    2306969493907701858,
    1153521859699409280,
    2254007511875588,
    12687767152414427136,
    9008713537683586,
    164399116042125312,
    1315103887499739200,
    24772082878251017,
    1188989888384471040,
    144124276262832640,
    9801521853783640080,
    1153489127519306752,
    5783255261986030084,
    9259968324145381506,
    9513590469632983681,
    9377059641948700736,
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
        magic_apply_rook_number_for_field(index);
    }

    for index in 0..64 {
        magic_apply_bishop_number_for_field(index);
    }

    let x1 = magic_get_rook_moves(0x101000040, 0);
    let x2 = magic_get_bishop_moves(0x8000200102000000, 18);
    let x3 = magic_get_rook_moves(0x101000040, 0);
}

pub fn magic_generate_rook_number_for_field(field_index: i32) -> u64 {
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

pub fn magic_generate_bishop_number_for_field(field_index: i32) -> u64 {
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

pub fn magic_get_rook_moves(bitboard: u64, field_index: i32) -> u64 {
    unsafe {
        let mut hash = bitboard & MAGIC_ROOK_FIELDS[field_index as usize].mask;
        hash = hash.wrapping_mul(MAGIC_ROOK_FIELDS[field_index as usize].magic_number);
        hash >>= 64 - MAGIC_ROOK_FIELDS[field_index as usize].shift;

        MAGIC_ROOK_FIELDS[field_index as usize].attacks[hash as usize]
    }
}

pub fn magic_get_bishop_moves(bitboard: u64, field_index: i32) -> u64 {
    unsafe {
        let mut hash = bitboard & MAGIC_BISHOP_FIELDS[field_index as usize].mask;
        hash = hash.wrapping_mul(MAGIC_BISHOP_FIELDS[field_index as usize].magic_number);
        hash >>= 64 - MAGIC_BISHOP_FIELDS[field_index as usize].shift;

        MAGIC_BISHOP_FIELDS[field_index as usize].attacks[hash as usize]
    }
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

fn magic_apply_rook_number_for_field(field_index: i32) {
    unsafe {
        let shift = MAGIC_ROOK_SHIFTS[field_index as usize];
        let mask = magic_get_rook_mask(field_index);
        let count = 1 << shift;

        let mut permutations = Vec::with_capacity(count as usize);
        let mut attacks = Vec::with_capacity(count as usize);

        for index in 0..count {
            let permutation = magic_get_permutation(mask, index as u64);

            permutations.push(permutation);
            attacks.push(magic_get_rook_attacks(permutation, field_index));
        }

        magic_apply_number_for_field(
            shift,
            count,
            mask,
            &permutations,
            &attacks,
            MAGIC_ROOK_NUMBERS[field_index as usize],
            &mut MAGIC_ROOK_FIELDS[field_index as usize],
        )
    }
}

fn magic_apply_bishop_number_for_field(field_index: i32) {
    unsafe {
        let shift = MAGIC_BISHOP_SHIFTS[field_index as usize];
        let mask = magic_get_bishop_mask(field_index);
        let count = 1 << shift;

        let mut permutations = Vec::with_capacity(count as usize);
        let mut attacks = Vec::with_capacity(count as usize);

        for index in 0..count {
            let permutation = magic_get_permutation(mask, index as u64);

            permutations.push(permutation);
            attacks.push(magic_get_bishop_attacks(permutation, field_index));
        }

        magic_apply_number_for_field(
            shift,
            count,
            mask,
            &permutations,
            &attacks,
            MAGIC_BISHOP_NUMBERS[field_index as usize],
            &mut MAGIC_BISHOP_FIELDS[field_index as usize],
        )
    }
}

fn magic_apply_number_for_field(
    shift: i32,
    count: i32,
    mask: u64,
    permutations: &Vec<u64>,
    attacks: &Vec<u64>,
    magic_number: u64,
    magic_field: &mut MagicField,
) {
    unsafe {
        magic_field.shift = shift;
        magic_field.mask = mask;
        magic_field.magic_number = magic_number;

        magic_field.attacks = Vec::with_capacity(count as usize);
        magic_field.attacks.resize(count as usize, 0);

        for index in 0..count {
            let permutation = permutations[index as usize];
            let attacks = attacks[index as usize];

            let hash = (permutation.wrapping_mul(magic_number)) >> (64 - shift);
            if magic_field.attacks[hash as usize] != 0 {
                panic!("Conflict");
            }

            magic_field.attacks[hash as usize] = attacks;
        }
    }
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
