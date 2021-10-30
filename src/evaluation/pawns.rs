use super::{parameters::*, taper_score};
use crate::cache::pawns::PawnHashTable;
use crate::engine::context::SearchStatistics;
use crate::state::board::Bitboard;
use crate::state::patterns::*;
use crate::state::*;
use std::cmp::{max, min};

pub fn evaluate(board: &Bitboard, pawns_table: &mut PawnHashTable, statistics: &mut SearchStatistics) -> i16 {
    let mut collision = false;
    match pawns_table.get(board.pawn_hash, &mut collision) {
        Some(entry) => {
            statistics.pawn_table_hits += 1;
            return entry.score;
        }
        None => {
            if collision {
                statistics.pawn_table_collisions += 1;
            }

            statistics.pawn_table_misses += 1;
        }
    }

    let score = evaluate_color(board, WHITE) - evaluate_color(board, BLACK);
    pawns_table.add(board.pawn_hash, score);
    statistics.pawn_table_added += 1;

    score
}

pub fn evaluate_without_cache(board: &Bitboard) -> i16 {
    evaluate_color(board, WHITE) - evaluate_color(board, BLACK)
}

fn evaluate_color(board: &Bitboard, color: u8) -> i16 {
    let mut doubled_pawns = 0;
    let mut isolated_pawns = 0;
    let mut chained_pawns = 0;
    let mut passing_pawns = 0;

    for file in 0..8 {
        let pawns_on_file_count = bit_count(get_file(file) & board.pieces[color as usize][PAWN as usize]);
        if pawns_on_file_count > 1 {
            doubled_pawns += pawns_on_file_count;
        }

        if pawns_on_file_count > 0 {
            let pawns_on_rail_count = bit_count(get_rail(file) & board.pieces[color as usize][PAWN as usize]);
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

        chained_pawns += bit_count(get_star(field_index as usize) & board.pieces[color as usize][PAWN as usize]);

        let enemy_pawns_ahead_count =
            bit_count(get_front(color as usize, field_index as usize) & board.pieces[(color ^ 1) as usize][PAWN as usize]);
        if enemy_pawns_ahead_count == 0 {
            passing_pawns += 1;
        }
    }

    let king = board.pieces[color as usize][KING as usize];
    let king_field = bit_scan(king);
    let fields_to_check = get_box(king_field as usize);
    let pawn_shield = bit_count(fields_to_check & board.pieces[color as usize][PAWN as usize]);
    let mut opened_files = 0;
    let king_field_file = (king_field % 8) as i8;
    for file in max(0, king_field_file - 1)..=(min(7, king_field_file + 1)) {
        if (get_file(file as usize) & board.pieces[color as usize][PAWN as usize]) == 0 {
            opened_files += 1;
        }
    }

    let game_phase = board.get_game_phase();
    let opening_score = 0
        + (doubled_pawns as i16) * DOUBLED_PAWN_OPENING
        + (isolated_pawns as i16) * ISOLATED_PAWN_OPENING
        + (chained_pawns as i16) * CHAINED_PAWN_OPENING
        + (passing_pawns as i16) * PASSING_PAWN_OPENING
        + (pawn_shield as i16) * PAWN_SHIELD_OPENING
        + (opened_files as i16) * PAWN_SHIELD_OPEN_FILE_OPENING;
    let ending_score = 0
        + (doubled_pawns as i16) * DOUBLED_PAWN_ENDING
        + (isolated_pawns as i16) * ISOLATED_PAWN_ENDING
        + (chained_pawns as i16) * CHAINED_PAWN_ENDING
        + (passing_pawns as i16) * PASSING_PAWN_ENDING
        + (pawn_shield as i16) * PAWN_SHIELD_ENDING
        + (opened_files as i16) * PAWN_SHIELD_OPEN_FILE_ENDING;

    taper_score(game_phase, opening_score, ending_score)
}
