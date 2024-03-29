use super::*;
use crate::cache::pawns::PawnHashTable;
use crate::engine::statistics::SearchStatistics;
use crate::state::representation::Board;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::conditional_expression;
use std::cmp;

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
pub fn evaluate<const DIAG: bool>(board: &Board, pawn_hashtable: &PawnHashTable, statistics: &mut SearchStatistics) -> i16 {
    match pawn_hashtable.get(board.pawn_hash) {
        Some(entry) => {
            conditional_expression!(DIAG, statistics.pawn_hashtable_hits += 1);
            return entry.score;
        }
        None => {
            conditional_expression!(DIAG, statistics.pawn_hashtable_misses += 1);
        }
    }

    let game_phase = board.game_phase;
    let initial_game_phase = board.evaluation_parameters.initial_game_phase;

    let score = evaluate_color(board, WHITE) - evaluate_color(board, BLACK);
    let score = score.taper_score(game_phase, initial_game_phase);

    pawn_hashtable.add(board.pawn_hash, score);
    conditional_expression!(DIAG, statistics.pawn_hashtable_added += 1);

    score
}

/// Does the same thing as [evaluate], but doesn't use pawn hashtable to save evalations.
pub fn evaluate_without_cache(board: &Board) -> EvaluationResult {
    evaluate_color(board, WHITE) - evaluate_color(board, BLACK)
}

/// Evaluates pawn structure on the `board` for the specified `color`.
fn evaluate_color(board: &Board, color: usize) -> EvaluationResult {
    let mut doubled_pawns = 0i16;
    let mut isolated_pawns = 0i16;
    let mut chained_pawns = 0i16;
    let mut passed_pawns = 0i16;
    let mut opened_files = 0i16;
    let mut pawn_shield = 0i16;

    for file in ALL_FILES {
        let pawns_on_file_count = (board.patterns.get_file(file) & board.pieces[color][PAWN]).bit_count() as i16;
        if pawns_on_file_count > 1 {
            doubled_pawns += pawns_on_file_count;
        }

        if pawns_on_file_count > 0 {
            let pawns_on_rail_count = (board.patterns.get_rail(file) & board.pieces[color][PAWN]).bit_count();
            if pawns_on_rail_count == 0 {
                isolated_pawns += 1;
            }
        }
    }

    let mut pawns_bb = board.pieces[color][PAWN];
    while pawns_bb != 0 {
        let square_bb = pawns_bb.get_lsb();
        let square = square_bb.bit_scan();
        pawns_bb = pawns_bb.pop_lsb();

        chained_pawns += (board.patterns.get_star(square) & board.pieces[color][PAWN]).bit_count() as i16;

        let front_bb = board.patterns.get_front(color, square);
        let enemy_pawns_ahead_count = (front_bb & board.pieces[color ^ 1][PAWN]).bit_count();
        let friendly_pawns_ahead_count = (front_bb & board.patterns.get_file(square) & board.pieces[color][PAWN]).bit_count();

        if enemy_pawns_ahead_count == 0 && friendly_pawns_ahead_count == 0 {
            passed_pawns += 1;
        }
    }

    let king_bb = board.pieces[color][KING];
    let king_square = king_bb.bit_scan();
    let king_square_file = (king_square & 7) as i8;
    pawn_shield = (board.patterns.get_box(king_square) & board.pieces[color][PAWN]).bit_count() as i16;

    for file in cmp::max(0, king_square_file - 1)..=(cmp::min(7, king_square_file + 1)) {
        if (board.patterns.get_file(file as usize) & board.pieces[color][PAWN]) == 0 {
            opened_files += 1;
        }
    }

    let opening_score = 0
        + doubled_pawns * board.evaluation_parameters.doubled_pawn_opening
        + isolated_pawns * board.evaluation_parameters.isolated_pawn_opening
        + chained_pawns * board.evaluation_parameters.chained_pawn_opening
        + passed_pawns * board.evaluation_parameters.passed_pawn_opening
        + pawn_shield * board.evaluation_parameters.pawn_shield_opening
        + opened_files * board.evaluation_parameters.pawn_shield_open_file_opening;
    let ending_score = 0
        + doubled_pawns * board.evaluation_parameters.doubled_pawn_ending
        + isolated_pawns * board.evaluation_parameters.isolated_pawn_ending
        + chained_pawns * board.evaluation_parameters.chained_pawn_ending
        + passed_pawns * board.evaluation_parameters.passed_pawn_ending
        + pawn_shield * board.evaluation_parameters.pawn_shield_ending
        + opened_files * board.evaluation_parameters.pawn_shield_open_file_ending;

    EvaluationResult::new(opening_score, ending_score)
}
