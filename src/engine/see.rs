use crate::state::*;
use std::cmp::max;

static mut TABLE: [[[[i8; 256]; 256]; 6]; 6] = [[[[0; 256]; 256]; 6]; 6];

pub fn init() {
    let test = evaluate(KNIGHT, KNIGHT, 70, 66);

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
    let mut gain = Vec::new();
    let attacking_piece_index = get_piece_index(attacking_piece);
    let defending_piece_index = get_piece_index(target_piece);

    evaluate_internal(&mut gain, attacking_piece_index, defending_piece_index, attackers, defenders);

    if gain.len() == 1 {
        return gain[0];
    }

    for i in (0..gain.len() - 1).rev() {
        gain[i] = -max(-gain[i], gain[i + 1]);
    }

    gain[0]
}

fn evaluate_internal(gain: &mut Vec<i8>, attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8) {
    let last_gain = if gain.is_empty() { 0 } else { gain[gain.len() - 1] } as i8;
    let target_piece_value = get_piece_value(target_piece) as i8;
    let new_gain = target_piece_value - last_gain;

    gain.push(new_gain);

    if defenders == 0 {
        return;
    }

    let new_attackers = pop_lsb(attackers as u64) as u8;
    let new_attacking_piece = bit_scan(get_lsb(defenders as u64)) as u8;

    evaluate_internal(gain, new_attacking_piece, attacking_piece, defenders, new_attackers);
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
