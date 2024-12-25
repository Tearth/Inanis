use super::PackedEval;
use crate::evaluation::params;
use crate::state::representation::Board;
use crate::state::*;
use crate::utils::assert_fast;
use crate::utils::bithelpers::BitHelpers;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoeff;

pub struct MaterialData {
    pub bishop_pair: i8,
    pub pawns_attacking_pieces: i8,
}

/// Evaluates material on the `board` and returns score from the white color perspective (more than 0 when advantage, less than 0 when disadvantage).
/// The piece values themself are included in PST so it's no longer evaluated here, instead other features like bishop pair are processed.
pub fn evaluate(board: &Board) -> PackedEval {
    let mut result = PackedEval::default();
    let white_data = get_material_data(board, WHITE);
    let black_data = get_material_data(board, BLACK);

    result += (white_data.bishop_pair - black_data.bishop_pair) * params::BISHOP_PAIR;
    result += (white_data.pawns_attacking_pieces - black_data.pawns_attacking_pieces) * params::PAWNS_ATTACKING_PIECES;

    result
}

/// Gets material data for `board` and `color`.
fn get_material_data(board: &Board, color: usize) -> MaterialData {
    assert_fast!(color < 2);

    let bishop_pair = if board.pieces[color][BISHOP].bit_count() == 2 { 1 } else { 0 };
    let enemy_pieces = board.occupancy[color ^ 1] & !board.pieces[color ^ 1][PAWN];
    let pawns_attacking_pieces = (board.pawn_attacks[color] & enemy_pieces).bit_count() as i8;

    MaterialData { bishop_pair, pawns_attacking_pieces }
}

/// Gets coefficients of material for `board` and inserts them into `coeffs`. Similarly, their indices (starting from `index`) are inserted into `indices`.
#[cfg(feature = "dev")]
pub fn get_coeffs(board: &Board, index: &mut u16, coeffs: &mut Vec<TunerCoeff>, indices: &mut Vec<u16>) {
    let white_data = get_material_data(board, WHITE);
    let black_data = get_material_data(board, BLACK);

    let mut data = [
        TunerCoeff::new(board.pieces[WHITE][PAWN].bit_count() as i8 - board.pieces[BLACK][PAWN].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][KNIGHT].bit_count() as i8 - board.pieces[BLACK][KNIGHT].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][BISHOP].bit_count() as i8 - board.pieces[BLACK][BISHOP].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][ROOK].bit_count() as i8 - board.pieces[BLACK][ROOK].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][QUEEN].bit_count() as i8 - board.pieces[BLACK][QUEEN].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][KING].bit_count() as i8 - board.pieces[BLACK][KING].bit_count() as i8, OPENING),
        TunerCoeff::new(board.pieces[WHITE][KING].bit_count() as i8 - board.pieces[BLACK][KING].bit_count() as i8, OPENING),
        TunerCoeff::new(white_data.bishop_pair - black_data.bishop_pair, OPENING),
        TunerCoeff::new(white_data.bishop_pair - black_data.bishop_pair, ENDING),
        TunerCoeff::new(white_data.pawns_attacking_pieces - black_data.pawns_attacking_pieces, OPENING),
        TunerCoeff::new(white_data.pawns_attacking_pieces - black_data.pawns_attacking_pieces, ENDING),
    ];

    for coeff in &mut data {
        let (value, _) = coeff.get_data();
        if value != 0 {
            coeffs.push(coeff.clone());
            indices.push(*index);
        }

        *index += 1;
    }
}
