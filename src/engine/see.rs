use crate::state::*;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::panic_fast;
use std::cmp;

pub const SEE_PAWN_VALUE: i8 = 2;
pub const SEE_KNISHOP_VALUE: i8 = 7;
pub const SEE_ROOK_VALUE: i8 = 10;
pub const SEE_QUEEN_VALUE: i8 = 22;
pub const SEE_KING_VALUE: i8 = 60;

pub struct SEEContainer {
    pub table: Box<[[[i8; 256]; 256]; 6]>,
}

impl SEEContainer {
    /// Gets a result of the static exchange evaluation, based on `attacking_piece`, `target_piece`, `attackers` and `defenders`.
    pub fn get(&self, attacking_piece: usize, target_piece: usize, attackers: usize, defenders: usize) -> i16 {
        let attacking_piece_index = self.get_see_piece_index(attacking_piece);
        let target_piece_index = self.get_see_piece_index(target_piece);
        let updated_attackers = attackers & !(1 << attacking_piece_index);

        let see = self.table[attacking_piece][defenders][updated_attackers];
        (self.get_piece_value(target_piece_index) - see) as i16 * 50
    }

    /// Evaluates a static exchange evaluation result, based on `target_piece`, `attackers`, `defenders`.
    fn evaluate(&self, target_piece: usize, attackers: usize, defenders: usize) -> i8 {
        if attackers == 0 {
            return 0;
        }

        let attacking_piece_index = attackers.get_lsb().bit_scan();
        let target_piece_index = self.get_see_piece_index(target_piece);

        self.evaluate_internal(attacking_piece_index, target_piece_index, attackers, defenders)
    }

    /// Recursive function called by `evaluate` to help evaluate a static exchange evaluation result.
    fn evaluate_internal(&self, attacking_piece: usize, target_piece: usize, attackers: usize, defenders: usize) -> i8 {
        if attackers == 0 {
            return 0;
        }

        let target_piece_value = self.get_piece_value(target_piece);
        let new_attackers = attackers & !(1 << attacking_piece);
        let new_attacking_piece = match defenders {
            0 => 0,
            _ => defenders.get_lsb().bit_scan(),
        };

        cmp::max(0, target_piece_value - self.evaluate_internal(new_attacking_piece, attacking_piece, defenders, new_attackers))
    }

    /// Converts `piece` index to SEE piece index, which supports multiple pieces of the same type stored in one variable:
    ///  - 1 pawn (index 0)
    ///  - 3 knights/bishops (index 1-3)
    ///  - 2 rooks (index 4-5)
    ///  - 1 queen (index 6)
    ///  - 1 king (index 7)
    fn get_see_piece_index(&self, piece: usize) -> usize {
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
    fn get_piece_value(&self, piece_index: usize) -> i8 {
        match piece_index {
            0 => SEE_PAWN_VALUE,            // Pawn
            1 | 2 | 3 => SEE_KNISHOP_VALUE, // 3x Knight/bishop
            4 | 5 => SEE_ROOK_VALUE,        // 2x Rook
            6 => SEE_QUEEN_VALUE,           // Queen
            7 => SEE_KING_VALUE,            // King
            _ => panic_fast!("Invalid value: piece_index={}", piece_index),
        }
    }
}

impl Default for SEEContainer {
    /// Constructs a default instance of [SEEContainer] with zeroed elements.
    fn default() -> Self {
        let mut result = Self { table: Box::new([[[0; 256]; 256]; 6]) };

        for target_piece in ALL_PIECES {
            for attackers in 0..256 {
                for defenders in 0..256 {
                    let see = result.evaluate(target_piece, attackers, defenders);
                    result.table[target_piece][attackers][defenders] = see;
                }
            }
        }

        result
    }
}
