use super::*;
use crate::cache::pawns::PawnHashTable;
use crate::engine::stats::SearchStatistics;
use crate::state::representation::Board;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::dev;
use std::cmp;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoefficient;

pub struct PawnsData {
    doubled_pawns: u8,
    isolated_pawns: u8,
    chained_pawns: u8,
    passed_pawns: u8,
    opened_files: u8,
    pawn_shield: u8,
}

/// Evaluates structure of pawns on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). This evaluator considers:
///  - doubled pawns (negative score)
///  - isolated pawns (negative score)
///  - chained pawns (positive score)
///  - passed pawns (positive score)
///  - open files next to the king (negative score)
///
/// To improve performance (using the fact that structure of pawns changes relatively rare), each evaluation is saved in the pawn hashtable,
/// and used again if possible.
pub fn evaluate(board: &Board, pawn_hashtable: &PawnHashTable, statistics: &mut SearchStatistics) -> PackedEval {
    match pawn_hashtable.get(board.state.pawn_hash) {
        Some(entry) => {
            dev!(statistics.pawn_hashtable_hits += 1);
            return PackedEval::new(entry.score_opening, entry.score_ending);
        }
        None => {
            dev!(statistics.pawn_hashtable_misses += 1);
        }
    }

    let white_evaluation = evaluate_color(board, WHITE);
    let black_evaluation = evaluate_color(board, BLACK);
    let eval = white_evaluation - black_evaluation;

    pawn_hashtable.add(board.state.pawn_hash, eval.get_opening(), eval.get_ending());
    dev!(statistics.pawn_hashtable_added += 1);

    eval
}

/// Does the same thing as [evaluate], but doesn't use pawn hashtable to save evalations.
pub fn evaluate_without_cache(board: &Board) -> PackedEval {
    evaluate_color(board, WHITE) - evaluate_color(board, BLACK)
}

/// Evaluates pawn structure on the `board` for the specified `color`.
fn evaluate_color(board: &Board, color: usize) -> PackedEval {
    let mut result = PackedEval::default();
    let pawns_data = get_pawns_data(board, color);

    result += params::DOUBLED_PAWN[pawns_data.doubled_pawns.min(7) as usize];
    result += params::ISOLATED_PAWN[pawns_data.isolated_pawns.min(7) as usize];
    result += params::CHAINED_PAWN[pawns_data.chained_pawns.min(7) as usize];
    result += params::PASSED_PAWN[pawns_data.passed_pawns.min(7) as usize];
    result += params::PAWN_SHIELD[pawns_data.pawn_shield.min(7) as usize];
    result += params::PAWN_SHIELD_OPEN_FILE[pawns_data.opened_files.min(7) as usize];

    result
}

/// Gets all pawn features on `board` for `color`.
fn get_pawns_data(board: &Board, color: usize) -> PawnsData {
    let mut doubled_pawns = 0;
    let mut isolated_pawns = 0;
    let mut chained_pawns = 0;
    let mut passed_pawns = 0;
    let mut pawn_shield = 0;
    let mut opened_files = 0;

    for file in ALL_FILES {
        let pawns_on_file = board.patterns.get_file(file) & board.pieces[color][PAWN];
        if pawns_on_file != 0 {
            let pawns_on_file_count = pawns_on_file.bit_count() as u8;

            if pawns_on_file_count > 1 {
                doubled_pawns += pawns_on_file_count - 1;
            }

            if (board.patterns.get_rail(file) & board.pieces[color][PAWN]) == 0 {
                isolated_pawns += 1;
            }
        }
    }

    let mut pawns_bb = board.pieces[color][PAWN];
    while pawns_bb != 0 {
        let square_bb = pawns_bb.get_lsb();
        let square = square_bb.bit_scan();
        pawns_bb = pawns_bb.pop_lsb();

        chained_pawns += ((board.patterns.get_front(color ^ 1, square) & board.patterns.get_star(square) & board.pieces[color][PAWN]) != 0) as u8;
        passed_pawns += ((board.patterns.get_front(color, square) & board.pieces[color ^ 1][PAWN]) == 0) as u8;
    }

    let king_bb = board.pieces[color][KING];
    let king_square = king_bb.bit_scan();
    let king_square_file = (king_square & 7) as i8;
    pawn_shield = (board.patterns.get_box(king_square) & board.pieces[color][PAWN]).bit_count() as u8;

    for file in cmp::max(0, king_square_file - 1)..=(cmp::min(7, king_square_file + 1)) {
        if (board.patterns.get_file(file as usize) & board.pieces[color][PAWN]) == 0 {
            opened_files += 1;
        }
    }

    PawnsData { doubled_pawns, isolated_pawns, chained_pawns, passed_pawns, pawn_shield, opened_files }
}

/// Gets coefficients of pawn structure on `board` and assigns indexes starting from `index`.
#[cfg(feature = "dev")]
pub fn get_coefficients(board: &Board, index: &mut u16, coefficients: &mut Vec<TunerCoefficient>, indices: &mut Vec<u16>) {
    let white_pawns_data = get_pawns_data(board, WHITE);
    let black_pawns_data = get_pawns_data(board, BLACK);

    get_array_coefficients(white_pawns_data.doubled_pawns, black_pawns_data.doubled_pawns, 8, index, coefficients, indices);
    get_array_coefficients(white_pawns_data.isolated_pawns, black_pawns_data.isolated_pawns, 8, index, coefficients, indices);
    get_array_coefficients(white_pawns_data.chained_pawns, black_pawns_data.chained_pawns, 8, index, coefficients, indices);
    get_array_coefficients(white_pawns_data.passed_pawns, black_pawns_data.passed_pawns, 8, index, coefficients, indices);
    get_array_coefficients(white_pawns_data.pawn_shield, black_pawns_data.pawn_shield, 8, index, coefficients, indices);
    get_array_coefficients(white_pawns_data.opened_files, black_pawns_data.opened_files, 8, index, coefficients, indices);
}
