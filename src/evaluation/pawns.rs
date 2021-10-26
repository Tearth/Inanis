use crate::cache::pawns::PawnsHashTable;
use crate::engine::context::SearchStatistics;
use crate::state::board::Bitboard;
use crate::state::patterns::*;
use crate::state::*;

pub fn evaluate(board: &Bitboard, pawns_table: &mut PawnsHashTable, statistics: &mut SearchStatistics) -> i16 {
    let entry = pawns_table.get(board.pawn_hash);
    if entry.key == (board.pawn_hash >> 48) as u16 {
        statistics.pawns_table_hits += 1;
        return entry.score;
    } else {
        if entry.key != 0 {
            statistics.pawns_table_misses += 1;
        }
    }

    let score = evaluate_color(board, WHITE) - evaluate_color(board, BLACK);
    pawns_table.add(board.pawn_hash, score);
    statistics.pawns_table_added_entries += 1;

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

    let game_phase = board.get_game_phase();
    let opening_score =
        (doubled_pawns as i16) * -20 + (isolated_pawns as i16) * -30 + (chained_pawns as i16) * 5 + (passing_pawns as i16) * 20;
    let ending_score =
        (doubled_pawns as i16) * -10 + (isolated_pawns as i16) * -5 + (chained_pawns as i16) * 0 + (passing_pawns as i16) * 60;

    ((game_phase * (opening_score as f32)) + ((1.0 - game_phase) * (ending_score as f32))) as i16
}
