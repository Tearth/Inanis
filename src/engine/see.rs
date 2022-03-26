use crate::evaluation::parameters;
use crate::state::*;
use std::cmp;

static mut TABLE: [[[i16; 256]; 256]; 6] = [[[0; 256]; 256]; 6];

/// Initializes static exchange evaluation table by filling it with every possible combination of target piece, attackers and defenders.
pub fn init() {
    for target_piece in 0..6 {
        for attackers in 0..256 {
            for defenders in 0..256 {
                unsafe { TABLE[target_piece][attackers][defenders] = evaluate(target_piece as u8, attackers as u8, defenders as u8) };
            }
        }
    }
}

/// Gets a result of the static exchange evaluation, based on `attacking_piece`, `target_piece`, `attackers` and `defenders`.
pub fn get(attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8) -> i16 {
    let attacking_piece_index = get_see_piece_index(attacking_piece);
    let target_piece_index = get_see_piece_index(target_piece);
    let updated_attackers = attackers & !(1 << attacking_piece_index);

    let see_result = unsafe { TABLE[attacking_piece as usize][defenders as usize][updated_attackers as usize] };
    get_piece_value(target_piece_index) - see_result
}

/// Evaluates a static exchange evaluation result, based on `target_piece`, `attackers`, `defenders`.
fn evaluate(target_piece: u8, attackers: u8, defenders: u8) -> i16 {
    if attackers == 0 {
        return 0;
    }

    let attacking_piece_index = bit_scan(get_lsb(attackers as u64)) as u8;
    let target_piece_index = get_see_piece_index(target_piece);

    evaluate_internal(attacking_piece_index, target_piece_index, attackers, defenders)
}

/// Recursive function called by [evaluate] to help evaluate a static exchange evaluation result.
fn evaluate_internal(attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8) -> i16 {
    if attackers == 0 {
        return 0;
    }

    let target_piece_value = get_piece_value(target_piece);
    let new_attackers = attackers & !(1 << attacking_piece);
    let new_attacking_piece = match defenders {
        0 => 0,
        _ => bit_scan(get_lsb(defenders as u64)) as u8,
    };

    let see_result = evaluate_internal(new_attacking_piece, attacking_piece, defenders, new_attackers);
    cmp::max(0, target_piece_value - see_result)
}

/// Converts `piece` index to SEE piece index, which supports multiple pieces of the same type stored in one variable:
///  - 1 pawn (index 0)
///  - 3 knights/bishops (index 1-3)
///  - 2 rooks (index 4-5)
///  - 1 queen (index 6)
///  - 1 king (index 7)
fn get_see_piece_index(piece: u8) -> u8 {
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

/// Gets a piece value based on `piece_index` saved in SEE format (look [get_see_piece_index]).
fn get_piece_value(piece_index: u8) -> i16 {
    unsafe {
        match piece_index {
            0 => parameters::PIECE_VALUE[PAWN as usize] as i16,           // Pawn
            1 | 2 | 3 => parameters::PIECE_VALUE[BISHOP as usize] as i16, // 3x Knight/bishop
            4 | 5 => parameters::PIECE_VALUE[ROOK as usize] as i16,       // 2x Rook
            6 => parameters::PIECE_VALUE[QUEEN as usize] as i16,          // Queen
            7 => parameters::PIECE_VALUE[KING as usize] as i16,           // King
            _ => panic!("Invalid value: piece_index={}", piece_index),
        }
    }
}
