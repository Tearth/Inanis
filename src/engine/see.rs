use crate::evaluation::EvaluationParameters;
use crate::state::*;
use std::cmp;
use std::sync::Arc;

pub struct SEEContainer {
    pub table: Box<[[[i16; 256]; 256]; 6]>,
}

impl SEEContainer {
    /// Constructs a default instance of [SEEContainer] with zeroed elements.
    pub fn new(evaluation_parameters: Option<Arc<EvaluationParameters>>) -> Self {
        let evaluation_parameters = evaluation_parameters.unwrap_or_else(|| Arc::new(Default::default()));
        let mut result = Self {
            table: Box::new([[[0; 256]; 256]; 6]),
        };

        for target_piece in 0..6 {
            for attackers in 0..256 {
                for defenders in 0..256 {
                    let see = result.evaluate(target_piece as u8, attackers as u8, defenders as u8, &evaluation_parameters);
                    result.table[target_piece][attackers][defenders] = see;
                }
            }
        }

        result
    }

    /// Gets a result of the static exchange evaluation, based on `attacking_piece`, `target_piece`, `attackers` and `defenders`.
    pub fn get(&self, attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8, evaluation_parameters: &EvaluationParameters) -> i16 {
        let attacking_piece_index = self.get_see_piece_index(attacking_piece);
        let target_piece_index = self.get_see_piece_index(target_piece);
        let updated_attackers = attackers & !(1 << attacking_piece_index);

        let see = self.table[attacking_piece as usize][defenders as usize][updated_attackers as usize];
        self.get_piece_value(target_piece_index, evaluation_parameters) - see
    }

    /// Evaluates a static exchange evaluation result, based on `target_piece`, `attackers`, `defenders`.
    fn evaluate(&self, target_piece: u8, attackers: u8, defenders: u8, evaluation_parameters: &EvaluationParameters) -> i16 {
        if attackers == 0 {
            return 0;
        }

        let attacking_piece_index = bit_scan(get_lsb(attackers as u64)) as u8;
        let target_piece_index = self.get_see_piece_index(target_piece);

        self.evaluate_internal(attacking_piece_index, target_piece_index, attackers, defenders, evaluation_parameters)
    }

    /// Recursive function called by `evaluate` to help evaluate a static exchange evaluation result.
    fn evaluate_internal(&self, attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8, evaluation_parameters: &EvaluationParameters) -> i16 {
        if attackers == 0 {
            return 0;
        }

        let target_piece_value = self.get_piece_value(target_piece, evaluation_parameters);
        let new_attackers = attackers & !(1 << attacking_piece);
        let new_attacking_piece = match defenders {
            0 => 0,
            _ => bit_scan(get_lsb(defenders as u64)) as u8,
        };

        let see = self.evaluate_internal(new_attacking_piece, attacking_piece, defenders, new_attackers, evaluation_parameters);
        cmp::max(0, target_piece_value - see)
    }

    /// Converts `piece` index to SEE piece index, which supports multiple pieces of the same type stored in one variable:
    ///  - 1 pawn (index 0)
    ///  - 3 knights/bishops (index 1-3)
    ///  - 2 rooks (index 4-5)
    ///  - 1 queen (index 6)
    ///  - 1 king (index 7)
    fn get_see_piece_index(&self, piece: u8) -> u8 {
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

    /// Gets a piece value based on `piece_index` saved in SEE format (look `get_see_piece_index`).
    fn get_piece_value(&self, piece_index: u8, evaluation_parameters: &EvaluationParameters) -> i16 {
        match piece_index {
            0 => evaluation_parameters.piece_value[PAWN as usize] as i16,           // Pawn
            1 | 2 | 3 => evaluation_parameters.piece_value[BISHOP as usize] as i16, // 3x Knight/bishop
            4 | 5 => evaluation_parameters.piece_value[ROOK as usize] as i16,       // 2x Rook
            6 => evaluation_parameters.piece_value[QUEEN as usize] as i16,          // Queen
            7 => evaluation_parameters.piece_value[KING as usize] as i16,           // King
            _ => panic!("Invalid value: piece_index={}", piece_index),
        }
    }
}
