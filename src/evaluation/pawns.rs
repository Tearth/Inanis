use super::*;
use crate::cache::pawns::PawnHashTable;
use crate::engine::context::SearchStatistics;
use crate::state::board::Bitboard;
use crate::utils::conditional_expression;
use std::cmp;

/// Evaluates structure of pawns on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). This evaluator considers:
///  - doubled pawns (negative score)
///  - isolated pawns (negative score)
///  - chained pawns (positive score)
///  - passing pawns (positive score)
///  - opened files next to the king (negative score)
///
/// To improve performance (using the fact that structure of pawns changes relatively rare), each evaluation is saved in the pawn hashtable,
/// and used again if possible.
pub fn evaluate<const DIAG: bool>(board: &Bitboard, pawn_hashtable: &PawnHashTable, statistics: &mut SearchStatistics) -> i16 {
    let mut collision = false;
    match pawn_hashtable.get(board.pawn_hash, &mut collision) {
        Some(entry) => {
            conditional_expression!(DIAG, statistics.pawn_hashtable_hits += 1);
            return entry.score;
        }
        None => {
            if collision {
                conditional_expression!(DIAG, statistics.pawn_hashtable_collisions += 1);
            }

            conditional_expression!(DIAG, statistics.pawn_hashtable_misses += 1);
        }
    }

    let score = evaluate_color(board, WHITE) - evaluate_color(board, BLACK);
    pawn_hashtable.add(board.pawn_hash, score);
    conditional_expression!(DIAG, statistics.pawn_hashtable_added += 1);

    score
}

/// Does the same thing as [evaluate], but doesn't use pawn hashtable to save evalations.
pub fn evaluate_without_cache(board: &Bitboard) -> i16 {
    evaluate_color(board, WHITE) - evaluate_color(board, BLACK)
}

/// Evaluates pawn structure on the `board` for the specified `color`.
fn evaluate_color(board: &Bitboard, color: u8) -> i16 {
    let mut doubled_pawns = 0;
    let mut isolated_pawns = 0;
    let mut chained_pawns = 0;
    let mut passing_pawns = 0;
    let mut opened_files = 0;

    for file in 0..8 {
        let pawns_on_file_count = bit_count(board.patterns.get_file(file) & board.pieces[color as usize][PAWN as usize]);
        if pawns_on_file_count > 1 {
            doubled_pawns += pawns_on_file_count;
        }

        if pawns_on_file_count > 0 {
            let pawns_on_rail_count = bit_count(board.patterns.get_rail(file) & board.pieces[color as usize][PAWN as usize]);
            if pawns_on_rail_count == 0 {
                isolated_pawns += 1;
            }
        }
    }

    let mut pawns = board.pieces[color as usize][PAWN as usize];
    while pawns != 0 {
        let field = get_lsb(pawns);
        let field_index = bit_scan(field);
        pawns = pop_lsb(pawns);

        chained_pawns += bit_count(board.patterns.get_star(field_index as usize) & board.pieces[color as usize][PAWN as usize]);

        let enemy_pawns_ahead_count =
            bit_count(board.patterns.get_front(color as usize, field_index as usize) & board.pieces[(color ^ 1) as usize][PAWN as usize]);
        if enemy_pawns_ahead_count == 0 {
            passing_pawns += 1;
        }
    }

    let king = board.pieces[color as usize][KING as usize];
    let king_field = bit_scan(king);
    let king_field_file = (king_field & 7) as i8;
    let pawn_shield = bit_count(board.patterns.get_box(king_field as usize) & board.pieces[color as usize][PAWN as usize]);

    for file in cmp::max(0, king_field_file - 1)..=(cmp::min(7, king_field_file + 1)) {
        if (board.patterns.get_file(file as usize) & board.pieces[color as usize][PAWN as usize]) == 0 {
            opened_files += 1;
        }
    }

    let game_phase = board.get_game_phase();
    let opening_score = 0
        + (doubled_pawns as i16) * board.evaluation_parameters.doubled_pawn_opening
        + (isolated_pawns as i16) * board.evaluation_parameters.isolated_pawn_opening
        + (chained_pawns as i16) * board.evaluation_parameters.chained_pawn_opening
        + (passing_pawns as i16) * board.evaluation_parameters.passing_pawn_opening
        + (pawn_shield as i16) * board.evaluation_parameters.pawn_shield_opening
        + (opened_files as i16) * board.evaluation_parameters.pawn_shield_open_file_opening;
    let ending_score = 0
        + (doubled_pawns as i16) * board.evaluation_parameters.doubled_pawn_ending
        + (isolated_pawns as i16) * board.evaluation_parameters.isolated_pawn_ending
        + (chained_pawns as i16) * board.evaluation_parameters.chained_pawn_ending
        + (passing_pawns as i16) * board.evaluation_parameters.passing_pawn_ending
        + (pawn_shield as i16) * board.evaluation_parameters.pawn_shield_ending
        + (opened_files as i16) * board.evaluation_parameters.pawn_shield_open_file_ending;

    taper_score(game_phase, opening_score, ending_score)
}
