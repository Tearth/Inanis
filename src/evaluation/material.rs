use crate::state::representation::Board;
use crate::state::*;
use crate::utils::bithelpers::BitHelpers;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoefficient;

/// Evaluates material on the `board` and returns score from the white color perspective (more than 0 when advantage, less than 0 when disadvantage).
/// This simple evaluator sums all scores of all present pieces using incremental counters in `board`, without considering the current game phase.
pub fn evaluate(board: &Board) -> i16 {
    board.material_scores[WHITE] - board.material_scores[BLACK]
}

/// Recalculates incremental counters on the `board`. This function should be called only once during board initialization, as it's too slow in regular search.
pub fn recalculate_incremental_values(board: &mut Board) {
    for color_index in ALL_COLORS {
        let mut score = 0;
        for piece_index in ALL_PIECES {
            let pieces_count = board.pieces[color_index][piece_index].bit_count();
            score += (pieces_count as i16) * board.evaluation_parameters.piece_value[piece_index];
        }

        board.material_scores[color_index] = score;
    }
}

#[cfg(feature = "dev")]
pub fn get_coefficients(board: &Board, index: &mut u16) -> Vec<TunerCoefficient> {
    let mut coefficients = vec![
        TunerCoefficient::new(board.pieces[WHITE][PAWN].bit_count() as i8 - board.pieces[BLACK][PAWN].bit_count() as i8, OPENING, 0),
        TunerCoefficient::new(board.pieces[WHITE][KNIGHT].bit_count() as i8 - board.pieces[BLACK][KNIGHT].bit_count() as i8, OPENING, 0),
        TunerCoefficient::new(board.pieces[WHITE][BISHOP].bit_count() as i8 - board.pieces[BLACK][BISHOP].bit_count() as i8, OPENING, 0),
        TunerCoefficient::new(board.pieces[WHITE][ROOK].bit_count() as i8 - board.pieces[BLACK][ROOK].bit_count() as i8, OPENING, 0),
        TunerCoefficient::new(board.pieces[WHITE][QUEEN].bit_count() as i8 - board.pieces[BLACK][QUEEN].bit_count() as i8, OPENING, 0),
        TunerCoefficient::new(board.pieces[WHITE][KING].bit_count() as i8 - board.pieces[BLACK][KING].bit_count() as i8, OPENING, 0),
    ];
    let mut coefficients_filtered = Vec::new();

    for coefficient in &mut coefficients {
        if coefficient.value != 0 {
            coefficient.index = *index;
            coefficients_filtered.push(coefficient.clone());
        }

        *index += 1;
    }

    coefficients_filtered
}
