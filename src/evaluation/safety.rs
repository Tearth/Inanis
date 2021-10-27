use std::cmp::{max, min};

use crate::state::board::Bitboard;
use crate::state::patterns::{get_box, get_file};
use crate::state::*;

pub fn evaluate(board: &Bitboard, white_attack_mask: u64, black_attack_mask: u64) -> i16 {
    evaluate_color(board, WHITE, black_attack_mask) - evaluate_color(board, BLACK, white_attack_mask)
}

fn evaluate_color(board: &Bitboard, color: u8, enemy_attack_mask: u64) -> i16 {
    let king = board.pieces[color as usize][KING as usize];
    let king_field = bit_scan(king);
    let fields_to_check = get_box(king_field as usize);
    let attacked_fields = bit_count(fields_to_check & enemy_attack_mask);
    let pawn_shield = bit_count(fields_to_check & board.pieces[color as usize][PAWN as usize]);

    let mut opened_files = 0;
    let king_field_file = (king_field % 8) as i8;
    for file in max(0, king_field_file - 1)..=(min(7, king_field_file + 1)) {
        if (get_file(file as usize) & board.pieces[color as usize][PAWN as usize]) == 0 {
            opened_files += 1;
        }
    }

    let game_phase = board.get_game_phase();
    let opening_score = (attacked_fields as i16) * -15 + (pawn_shield as i16) * 10 + (opened_files as i16) * -15;
    let ending_score = (attacked_fields as i16) * -5 + (pawn_shield as i16) * 0 + (opened_files as i16) * 0;

    ((game_phase * (opening_score as f32)) + ((1.0 - game_phase) * (ending_score as f32))) as i16
}
