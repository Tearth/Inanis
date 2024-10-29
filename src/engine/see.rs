use crate::state::*;
use crate::utils::assert_fast;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::panic_fast;
use std::alloc;
use std::alloc::Layout;
use std::cmp;
use std::mem;
use std::sync::OnceLock;

pub const SEE_PAWN_VALUE: i8 = 2;
pub const SEE_KNISHOP_VALUE: i8 = 7;
pub const SEE_ROOK_VALUE: i8 = 10;
pub const SEE_QUEEN_VALUE: i8 = 22;
pub const SEE_KING_VALUE: i8 = 60;

static SEE_TABLE: OnceLock<Box<[[[i8; 256]; 256]; 6]>> = OnceLock::new();

/// Initializes static exchange evaluation table.
pub fn init() {
    const SIZE: usize = mem::size_of::<i8>();
    unsafe {
        let ptr = alloc::alloc_zeroed(Layout::from_size_align(256 * 256 * 6 * SIZE, SIZE).unwrap());
        let mut table = Box::from_raw(ptr as *mut [[[i8; 256]; 256]; 6]);

        for target_piece in ALL_PIECES {
            for attackers in 0..256 {
                for defenders in 0..256 {
                    table[target_piece][attackers][defenders] = evaluate(target_piece, attackers, defenders);
                }
            }
        }

        let _ = SEE_TABLE.set(table);
    }
}

/// Gets a result of the static exchange evaluation, based on `attacking_piece`, `target_piece`, `attackers` and `defenders`.
pub fn get(attacking_piece: usize, target_piece: usize, attackers: usize, defenders: usize) -> i16 {
    assert_fast!(attacking_piece <= 6);
    assert_fast!(target_piece <= 6);
    assert_fast!(attackers != 0);

    let attacking_piece_index = get_see_piece_index(attacking_piece);
    let target_piece_index = get_see_piece_index(target_piece);
    let updated_attackers = attackers & !(1 << attacking_piece_index);

    let table = unsafe { SEE_TABLE.get().unwrap_unchecked() };
    let see = table[attacking_piece][defenders][updated_attackers];
    (get_piece_value(target_piece_index) - see) as i16 * 50
}

/// Evaluates a static exchange evaluation result, based on `target_piece`, `attackers`, `defenders`.
fn evaluate(target_piece: usize, attackers: usize, defenders: usize) -> i8 {
    assert_fast!(target_piece <= 6);

    if attackers == 0 {
        return 0;
    }

    let attacking_piece_index = attackers.get_lsb().bit_scan();
    let target_piece_index = get_see_piece_index(target_piece);

    evaluate_internal(attacking_piece_index, target_piece_index, attackers, defenders)
}

/// Recursive function called by `evaluate` to help evaluate a static exchange evaluation result.
fn evaluate_internal(attacking_piece: usize, target_piece: usize, attackers: usize, defenders: usize) -> i8 {
    assert_fast!(target_piece < 8);

    if attackers == 0 {
        return 0;
    }

    let target_piece_value = get_piece_value(target_piece);
    let new_attackers = attackers & !(1 << attacking_piece);
    let new_attacking_piece = match defenders {
        0 => 0,
        _ => defenders.get_lsb().bit_scan(),
    };

    cmp::max(0, target_piece_value - evaluate_internal(new_attacking_piece, attacking_piece, defenders, new_attackers))
}

/// Converts `piece` index to SEE piece index, which supports multiple pieces of the same type stored in one variable:
///  - 1 pawn (index 0)
///  - 3 knights/bishops (index 1-3)
///  - 2 rooks (index 4-5)
///  - 1 queen (index 6)
///  - 1 king (index 7)
fn get_see_piece_index(piece: usize) -> usize {
    assert_fast!(piece < 6);

    match piece {
        PAWN => 0,
        KNIGHT => 1,
        BISHOP => 1,
        ROOK => 4,
        QUEEN => 6,
        KING => 7,
        _ => panic_fast!("Invalid value: piece={}", piece),
    }
}

/// Gets a piece value based on `piece_index` saved in SEE format (look `get_see_piece_index`).
fn get_piece_value(piece_index: usize) -> i8 {
    assert_fast!(piece_index < 8);

    match piece_index {
        0 => SEE_PAWN_VALUE,            // Pawn
        1 | 2 | 3 => SEE_KNISHOP_VALUE, // 3x Knight/bishop
        4 | 5 => SEE_ROOK_VALUE,        // 2x Rook
        6 => SEE_QUEEN_VALUE,           // Queen
        7 => SEE_KING_VALUE,            // King
        _ => panic_fast!("Invalid value: piece_index={}", piece_index),
    }
}
