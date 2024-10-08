use super::*;
use crate::state::movescan;
use crate::state::representation::Board;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoeff;
use crate::utils::assert_fast;

pub struct MobilityData {
    knight_mobility: PieceMobility,
    bishop_mobility: PieceMobility,
    rook_mobility: PieceMobility,
    queen_mobility: PieceMobility,
}

#[derive(Default)]
pub struct EvalAux {
    pub king_area_threats: i8,
}

pub struct PieceMobility {
    pub inner: i8,
    pub outer: i8,
}

/// Evaluates mobility and part of the king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). This evaluator does two things at once: first, counts all possible moves of knight, bishop, rook, queen
/// (pawns and king are too slow and not very important), and second, sums how many squares around both kings are dangered by enemy side
/// (`dangered_white_king_squares` and `dangered_black_king_squares`). This is used in the safety evaluator, to prevent calculating the same thing twice.
pub fn evaluate(board: &Board, white_aux: &mut EvalAux, black_aux: &mut EvalAux) -> PackedEval {
    let mut result = PackedEval::default();
    let white_data = get_mobility_data(board, WHITE, white_aux);
    let black_data = get_mobility_data(board, BLACK, black_aux);

    result += (white_data.knight_mobility.inner - black_data.knight_mobility.inner) * params::MOBILITY_INNER[KNIGHT];
    result += (white_data.bishop_mobility.inner - black_data.bishop_mobility.inner) * params::MOBILITY_INNER[BISHOP];
    result += (white_data.rook_mobility.inner - black_data.rook_mobility.inner) * params::MOBILITY_INNER[ROOK];
    result += (white_data.queen_mobility.inner - black_data.queen_mobility.inner) * params::MOBILITY_INNER[QUEEN];

    result += (white_data.knight_mobility.outer - black_data.knight_mobility.outer) * params::MOBILITY_OUTER[KNIGHT];
    result += (white_data.bishop_mobility.outer - black_data.bishop_mobility.outer) * params::MOBILITY_OUTER[BISHOP];
    result += (white_data.rook_mobility.outer - black_data.rook_mobility.outer) * params::MOBILITY_OUTER[ROOK];
    result += (white_data.queen_mobility.outer - black_data.queen_mobility.outer) * params::MOBILITY_OUTER[QUEEN];

    result
}

fn get_mobility_data(board: &Board, color: usize, aux: &mut EvalAux) -> MobilityData {
    assert_fast!(color < 2);

    MobilityData {
        knight_mobility: movescan::get_piece_mobility::<KNIGHT>(board, color, aux),
        bishop_mobility: movescan::get_piece_mobility::<BISHOP>(board, color, aux),
        rook_mobility: movescan::get_piece_mobility::<ROOK>(board, color, aux),
        queen_mobility: movescan::get_piece_mobility::<QUEEN>(board, color, aux),
    }
}

/// Gets coefficients of mobility for `board` and inserts them into `coefficients`. Similarly, their indices (starting from `index`) are inserted into `indices`.
/// Some additional data is also saved in `white_aux` and `black_aux` for further processing.
#[cfg(feature = "dev")]
pub fn get_coeffs(board: &Board, white_aux: &mut EvalAux, black_aux: &mut EvalAux, index: &mut u16, coeffs: &mut Vec<TunerCoeff>, indices: &mut Vec<u16>) {
    let white_data = get_mobility_data(board, WHITE, white_aux);
    let black_data = get_mobility_data(board, BLACK, black_aux);

    let mut data = [
        TunerCoeff::new(0, OPENING),
        TunerCoeff::new(0, ENDING),
        TunerCoeff::new(white_data.knight_mobility.inner - black_data.knight_mobility.inner, OPENING),
        TunerCoeff::new(white_data.knight_mobility.inner - black_data.knight_mobility.inner, ENDING),
        TunerCoeff::new(white_data.bishop_mobility.inner - black_data.bishop_mobility.inner, OPENING),
        TunerCoeff::new(white_data.bishop_mobility.inner - black_data.bishop_mobility.inner, ENDING),
        TunerCoeff::new(white_data.rook_mobility.inner - black_data.rook_mobility.inner, OPENING),
        TunerCoeff::new(white_data.rook_mobility.inner - black_data.rook_mobility.inner, ENDING),
        TunerCoeff::new(white_data.queen_mobility.inner - black_data.queen_mobility.inner, OPENING),
        TunerCoeff::new(white_data.queen_mobility.inner - black_data.queen_mobility.inner, ENDING),
        TunerCoeff::new(0, OPENING),
        TunerCoeff::new(0, ENDING),
        //
        TunerCoeff::new(0, OPENING),
        TunerCoeff::new(0, ENDING),
        TunerCoeff::new(white_data.knight_mobility.outer - black_data.knight_mobility.outer, OPENING),
        TunerCoeff::new(white_data.knight_mobility.outer - black_data.knight_mobility.outer, ENDING),
        TunerCoeff::new(white_data.bishop_mobility.outer - black_data.bishop_mobility.outer, OPENING),
        TunerCoeff::new(white_data.bishop_mobility.outer - black_data.bishop_mobility.outer, ENDING),
        TunerCoeff::new(white_data.rook_mobility.outer - black_data.rook_mobility.outer, OPENING),
        TunerCoeff::new(white_data.rook_mobility.outer - black_data.rook_mobility.outer, ENDING),
        TunerCoeff::new(white_data.queen_mobility.outer - black_data.queen_mobility.outer, OPENING),
        TunerCoeff::new(white_data.queen_mobility.outer - black_data.queen_mobility.outer, ENDING),
        TunerCoeff::new(0, OPENING),
        TunerCoeff::new(0, ENDING),
    ];

    for coeff in &mut data {
        let (value, _) = coeff.get_data();
        if value != 0 {
            indices.push(*index);
            coeffs.push(coeff.clone());
        }

        *index += 1;
    }
}
