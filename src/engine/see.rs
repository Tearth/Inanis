use crate::evaluation::EvaluationParameters;
use crate::state::*;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::panic_fast;
use std::cmp;
use std::sync::Arc;

pub struct SEEContainer {
    pub table: Vec<[[i16; 256]; 256]>,
    pub evaluation_parameters: Arc<EvaluationParameters>,
}

impl SEEContainer {
    /// Constructs a default instance of [SEEContainer] with zeroed elements.
    pub fn new(evaluation_parameters: Option<Arc<EvaluationParameters>>) -> Self {
        let mut table = Vec::new();
        table.resize(6, [[0; 256]; 256]);

        let evaluation_parameters = evaluation_parameters.unwrap_or_else(|| Arc::new(Default::default()));
        let mut result = Self { table, evaluation_parameters };

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

    /// Gets a result of the static exchange evaluation, based on `attacking_piece`, `target_piece`, `attackers` and `defenders`.
    pub fn get(&self, attacking_piece: usize, target_piece: usize, attackers: usize, defenders: usize) -> i16 {
        let attacking_piece_index = self.get_see_piece_index(attacking_piece);
        let target_piece_index = self.get_see_piece_index(target_piece);
        let updated_attackers = attackers & !(1 << attacking_piece_index);

        let see = self.table[attacking_piece][defenders][updated_attackers];
        self.get_piece_value(target_piece_index) - see
    }

    /// Evaluates a static exchange evaluation result, based on `target_piece`, `attackers`, `defenders`.
    fn evaluate(&self, target_piece: usize, attackers: usize, defenders: usize) -> i16 {
        if attackers == 0 {
            return 0;
        }

        let attacking_piece_index = attackers.get_lsb().bit_scan();
        let target_piece_index = self.get_see_piece_index(target_piece);

        self.evaluate_internal(attacking_piece_index, target_piece_index, attackers, defenders)
    }

    /// Recursive function called by `evaluate` to help evaluate a static exchange evaluation result.
    fn evaluate_internal(&self, attacking_piece: usize, target_piece: usize, attackers: usize, defenders: usize) -> i16 {
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
    fn get_piece_value(&self, piece_index: usize) -> i16 {
        match piece_index {
            0 => self.evaluation_parameters.piece_value[PAWN],           // Pawn
            1 | 2 | 3 => self.evaluation_parameters.piece_value[BISHOP], // 3x Knight/bishop
            4 | 5 => self.evaluation_parameters.piece_value[ROOK],       // 2x Rook
            6 => self.evaluation_parameters.piece_value[QUEEN],          // Queen
            7 => self.evaluation_parameters.piece_value[KING],           // King
            _ => panic_fast!("Invalid value: piece_index={}", piece_index),
        }
    }
}
