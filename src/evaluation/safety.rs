use super::*;
use crate::state::representation::Board;
use crate::utils::assert_fast;
use crate::utils::bithelpers::BitHelpers;
use mobility::EvalAux;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoeff;

pub struct SafetyData {
    pub knight_safe_checks: u8,
    pub bishop_safe_checks: u8,
    pub rook_safe_checks: u8,
    pub queen_safe_checks: u8,
}

/// Evaluates king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). Both additional parameters, `white_aux` and `black_aux`, are
/// calculated during mobility evaluation and are used here to get the final score.
pub fn evaluate(board: &Board, white_aux: &EvalAux, black_aux: &EvalAux) -> PackedEval {
    evaluate_color(board, WHITE, white_aux, black_aux) - evaluate_color(board, BLACK, white_aux, black_aux)
}

/// Evaluates kibg safety on the `board` for the specified `color``, using `white_aux` and `black_aux`.
pub fn evaluate_color(board: &Board, color: usize, white_aux: &EvalAux, black_aux: &EvalAux) -> PackedEval {
    assert_fast!(color < 2);

    let mut result = PackedEval::default();
    let (stm_aux, nstm_aux) = match color {
        WHITE => (white_aux, black_aux),
        BLACK => (black_aux, white_aux),
        _ => panic_fast!("Invalid value: color={}", color),
    };
    let data = get_safety_data(board, color, stm_aux, nstm_aux);

    result += params::KING_AREA_THREATS[((stm_aux.king_area_threats) as usize).min(7)];
    result += params::KNIGHT_SAFE_CHECKS[((data.knight_safe_checks) as usize).min(7)];
    result += params::BISHOP_SAFE_CHECKS[((data.bishop_safe_checks) as usize).min(7)];
    result += params::ROOK_SAFE_CHECKS[((data.rook_safe_checks) as usize).min(7)];
    result += params::QUEEN_SAFE_CHECKS[((data.queen_safe_checks) as usize).min(7)];

    result
}

/// Gets safety data for `board`, `color`, `our_aux` and `their_aux`.
pub fn get_safety_data(board: &Board, color: usize, our_aux: &EvalAux, their_aux: &EvalAux) -> SafetyData {
    assert_fast!(color < 2);

    let occupancy_bb = board.occupancy[WHITE] | board.occupancy[BLACK];
    let enemy_king_square = (board.pieces[color ^ 1][KING]).bit_scan();

    let threats = their_aux.knight_threats | their_aux.bishop_threats | their_aux.rook_threats | their_aux.queen_threats | board.pawn_attacks[color ^ 1];
    let knight_moves_bb = movegen::get_knight_moves(enemy_king_square);
    let bishop_moves_bb = movegen::get_bishop_moves(occupancy_bb, enemy_king_square);
    let rook_moves_bb = movegen::get_rook_moves(occupancy_bb, enemy_king_square);
    let queen_moves_bb = movegen::get_queen_moves(occupancy_bb, enemy_king_square);
    let king_moves_bb = movegen::get_king_moves(enemy_king_square);

    let knight_safe_checks = ((knight_moves_bb & our_aux.knight_threats) & !threats & !king_moves_bb).bit_count() as u8;
    let bishop_safe_checks = ((bishop_moves_bb & our_aux.bishop_threats) & !threats & !king_moves_bb).bit_count() as u8;
    let rook_safe_checks = ((rook_moves_bb & our_aux.rook_threats) & !threats & !king_moves_bb).bit_count() as u8;
    let queen_safe_checks = ((queen_moves_bb & our_aux.queen_threats) & !threats & !king_moves_bb).bit_count() as u8;

    SafetyData { knight_safe_checks, bishop_safe_checks, rook_safe_checks, queen_safe_checks }
}

/// Gets coefficients of king safety for `board` and inserts them into `coeffs`. Similarly, their indices (starting from `index`) are inserted into `indices`.
/// Additionally, `white_aux` and `black_aux` calculated during mobility phase are also used here.
#[cfg(feature = "dev")]
pub fn get_coeffs(board: &Board, white_aux: &EvalAux, black_aux: &EvalAux, index: &mut u16, coeffs: &mut Vec<TunerCoeff>, indices: &mut Vec<u16>) {
    let white_data = get_safety_data(board, WHITE, white_aux, black_aux);
    let black_data = get_safety_data(board, BLACK, black_aux, white_aux);

    get_array_coeffs(white_aux.king_area_threats as u8, black_aux.king_area_threats as u8, 8, index, coeffs, indices);
    get_array_coeffs(white_data.knight_safe_checks, black_data.knight_safe_checks, 8, index, coeffs, indices);
    get_array_coeffs(white_data.bishop_safe_checks, black_data.bishop_safe_checks, 8, index, coeffs, indices);
    get_array_coeffs(white_data.rook_safe_checks, black_data.rook_safe_checks, 8, index, coeffs, indices);
    get_array_coeffs(white_data.queen_safe_checks, black_data.queen_safe_checks, 8, index, coeffs, indices);
}
