use crate::state::*;
use std::cmp::max;

static mut TABLE: [[[[i8; 256]; 256]; 6]; 6] = [[[[0; 256]; 256]; 6]; 6];

pub fn init() {
    for attacking_piece in 0..6 {
        let attacker_index = get_piece_index(attacking_piece as u8);
        for target_piece in 0..6 {
            for attackers in 0..256 {
                if ((1 << attacker_index) & attackers) == 0 {
                    continue;
                }

                for defenders in 0..256 {
                    unsafe {
                        TABLE[attacking_piece][target_piece][attackers][defenders] =
                            evaluate(attacking_piece as u8, target_piece as u8, attackers as u8, defenders as u8)
                    };
                }
            }
        }
    }
}

pub fn get(attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8) -> i8 {
    unsafe { TABLE[attacking_piece as usize][target_piece as usize][attackers as usize][defenders as usize] }
}

fn evaluate(attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8) -> i8 {
    let attacking_piece_index = get_piece_index(attacking_piece);
    let defending_piece_index = get_piece_index(target_piece);

    evaluate_internal(true, attacking_piece_index, defending_piece_index, attackers, defenders)
}

fn evaluate_internal(force: bool, attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8) -> i8 {
    if attackers == 0 {
        return 0;
    }

    let target_piece_value = get_piece_value(target_piece) as i8;
    let new_attackers = attackers & !(1 << attacking_piece);
    let new_attacking_piece = match defenders {
        0 => 0,
        _ => bit_scan(get_lsb(defenders as u64)) as u8,
    };

    let evaluation = evaluate_internal(false, new_attacking_piece, attacking_piece, defenders, new_attackers);
    let result = target_piece_value - evaluation;

    match force {
        true => result,
        false => max(0, target_piece_value - evaluation),
    }
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
