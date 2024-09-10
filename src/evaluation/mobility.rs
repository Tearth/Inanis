use super::*;
use crate::state::movescan;
use crate::state::representation::Board;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoefficient;

pub struct MobilityData {
    knight_mobility: PieceMobility,
    bishop_mobility: PieceMobility,
    rook_mobility: PieceMobility,
    queen_mobility: PieceMobility,
}

#[derive(Default)]
pub struct MobilityAuxData {
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
pub fn evaluate(board: &Board, white_aux: &mut MobilityAuxData, black_aux: &mut MobilityAuxData) -> PackedEval {
    let mut result = PackedEval::new(0, 0);
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

fn get_mobility_data(board: &Board, color: usize, aux: &mut MobilityAuxData) -> MobilityData {
    MobilityData {
        knight_mobility: movescan::get_piece_mobility::<KNIGHT>(board, color, aux),
        bishop_mobility: movescan::get_piece_mobility::<BISHOP>(board, color, aux),
        rook_mobility: movescan::get_piece_mobility::<ROOK>(board, color, aux),
        queen_mobility: movescan::get_piece_mobility::<QUEEN>(board, color, aux),
    }
}

/// Gets coefficients of mobility on `board` and assigns indexes starting from `index`. Similarly to [evaluate], both `dangered_white_king_squares` and
/// `dangered_black_king_squares` are accordingly updated.
#[cfg(feature = "dev")]
pub fn get_coefficients(
    board: &Board,
    white_aux: &mut MobilityAuxData,
    black_aux: &mut MobilityAuxData,
    index: &mut u16,
    coefficients: &mut Vec<TunerCoefficient>,
    indices: &mut Vec<u16>,
) {
    let white_data = get_mobility_data(board, WHITE, white_aux);
    let black_data = get_mobility_data(board, BLACK, black_aux);

    let mut data = [
        TunerCoefficient::new(0, OPENING),
        TunerCoefficient::new(0, ENDING),
        TunerCoefficient::new(white_data.knight_mobility.inner - black_data.knight_mobility.inner, OPENING),
        TunerCoefficient::new(white_data.knight_mobility.inner - black_data.knight_mobility.inner, ENDING),
        TunerCoefficient::new(white_data.bishop_mobility.inner - black_data.bishop_mobility.inner, OPENING),
        TunerCoefficient::new(white_data.bishop_mobility.inner - black_data.bishop_mobility.inner, ENDING),
        TunerCoefficient::new(white_data.rook_mobility.inner - black_data.rook_mobility.inner, OPENING),
        TunerCoefficient::new(white_data.rook_mobility.inner - black_data.rook_mobility.inner, ENDING),
        TunerCoefficient::new(white_data.queen_mobility.inner - black_data.queen_mobility.inner, OPENING),
        TunerCoefficient::new(white_data.queen_mobility.inner - black_data.queen_mobility.inner, ENDING),
        TunerCoefficient::new(0, OPENING),
        TunerCoefficient::new(0, ENDING),
        //
        TunerCoefficient::new(0, OPENING),
        TunerCoefficient::new(0, ENDING),
        TunerCoefficient::new(white_data.knight_mobility.outer - black_data.knight_mobility.outer, OPENING),
        TunerCoefficient::new(white_data.knight_mobility.outer - black_data.knight_mobility.outer, ENDING),
        TunerCoefficient::new(white_data.bishop_mobility.outer - black_data.bishop_mobility.outer, OPENING),
        TunerCoefficient::new(white_data.bishop_mobility.outer - black_data.bishop_mobility.outer, ENDING),
        TunerCoefficient::new(white_data.rook_mobility.outer - black_data.rook_mobility.outer, OPENING),
        TunerCoefficient::new(white_data.rook_mobility.outer - black_data.rook_mobility.outer, ENDING),
        TunerCoefficient::new(white_data.queen_mobility.outer - black_data.queen_mobility.outer, OPENING),
        TunerCoefficient::new(white_data.queen_mobility.outer - black_data.queen_mobility.outer, ENDING),
        TunerCoefficient::new(0, OPENING),
        TunerCoefficient::new(0, ENDING),
    ];

    for coefficient in &mut data {
        let (value, _) = coefficient.get_data();
        if value != 0 {
            indices.push(*index);
            coefficients.push(coefficient.clone());
        }

        *index += 1;
    }
}
