use crate::state::*;
use std::cmp::*;

static mut TABLE: [[[i8; 256]; 256]; 6] = [[[0; 256]; 256]; 6];

pub fn init() {
    for target_piece in 0..6 {
        for attackers in 0..256 {
            for defenders in 0..256 {
                unsafe { TABLE[target_piece][attackers][defenders] = evaluate(target_piece as u8, attackers as u8, defenders as u8) };
            }
        }
    }
}

pub fn get(attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8) -> i8 {
    let attacking_piece_index = get_piece_index(attacking_piece);
    let target_piece_index = get_piece_index(target_piece);
    let updated_attackers = attackers & !(1 << attacking_piece_index);

    let see_result = unsafe { TABLE[attacking_piece as usize][defenders as usize][updated_attackers as usize] };
    (get_piece_value(target_piece_index) as i8) - see_result
}

fn evaluate(target_piece: u8, attackers: u8, defenders: u8) -> i8 {
    if attackers == 0 {
        return 0;
    }

    let attacking_piece_index = bit_scan(get_lsb(attackers as u64)) as u8;
    let target_piece_index = get_piece_index(target_piece);

    evaluate_internal(attacking_piece_index, target_piece_index, attackers, defenders)
}

fn evaluate_internal(attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8) -> i8 {
    if attackers == 0 {
        return 0;
    }

    let target_piece_value = get_piece_value(target_piece) as i8;
    let new_attackers = attackers & !(1 << attacking_piece);
    let new_attacking_piece = match defenders {
        0 => 0,
        _ => bit_scan(get_lsb(defenders as u64)) as u8,
    };

    let see_result = evaluate_internal(new_attacking_piece, attacking_piece, defenders, new_attackers);
    max(0, target_piece_value - see_result)
}

fn get_piece_index(piece: u8) -> u8 {
    match piece {
        PAWN => 0,
        KNIGHT => 1,
        BISHOP => 1,
        ROOK => 4,
        QUEEN => 6,
        KING => 7,
        _ => panic!("Invalid value: piece={}", piece),
    }
}

fn get_piece_value(piece_index: u8) -> u8 {
    match piece_index {
        0 => 1,         // Pawn
        1 | 2 | 3 => 3, // 3x Knight/bishop
        4 | 5 => 5,     // 2x Rook
        6 => 9,         // Queen
        7 => 32,        // King
        _ => panic!("Invalid value: piece_index={}", piece_index),
    }
}
