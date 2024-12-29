use super::*;
use crate::state::movescan;
use crate::state::representation::Board;
use crate::utils::assert_fast;
use crate::utils::bithelpers::BitHelpers;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoeff;

pub struct MobilityData {
    rook_open_file: i8,
    rook_semi_open_file: i8,
    knight_mobility: PieceMobility,
    bishop_mobility: PieceMobility,
    rook_mobility: PieceMobility,
    queen_mobility: PieceMobility,
}

#[derive(Default)]
pub struct EvalAux {
    pub king_area_threats: i8,
    pub knight_threats: u64,
    pub bishop_threats: u64,
    pub rook_threats: u64,
    pub queen_threats: u64,
}

pub struct PieceMobility {
    pub inner: i8,
    pub outer: i8,
}

/// Evaluates mobility and part of the king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). This evaluator does two things at once: first, counts all possible moves of knight, bishop, rook, queen
/// (pawns and king are too slow and not very important), and second, fills `white_aux` and `black_aux` with additional data used in other evaluators.
pub fn evaluate(board: &Board, white_aux: &mut EvalAux, black_aux: &mut EvalAux) -> PackedEval {
    let mut result = PackedEval::default();
    let white_data = get_mobility_data(board, WHITE, white_aux);
    let black_data = get_mobility_data(board, BLACK, black_aux);

    result += (white_data.rook_open_file - black_data.rook_open_file) * params::ROOK_OPEN_FILE;
    result += (white_data.rook_semi_open_file - black_data.rook_semi_open_file) * params::ROOK_SEMI_OPEN_FILE;

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

/// Gets mobility data for `board`, `color` and fills `aux` with additional data used in other evaluators.
fn get_mobility_data(board: &Board, color: usize, aux: &mut EvalAux) -> MobilityData {
    assert_fast!(color < 2);

    let mut rook_open_file = 0;
    let mut rook_semi_open_file = 0;
    let mut rooks_bb = board.pieces[color][ROOK];

    while rooks_bb != 0 {
        let square_bb = rooks_bb.get_lsb();
        let square = square_bb.bit_scan();
        rooks_bb = rooks_bb.pop_lsb();

        let file = patterns::get_file(square);
        if (file & board.pieces[color][PAWN]) == 0 {
            if (file & board.pieces[color ^ 1][PAWN]) == 0 {
                rook_open_file += 1;
            } else {
                rook_semi_open_file += 1;
            }
        }
    }

    MobilityData {
        rook_open_file,
        rook_semi_open_file,

        knight_mobility: movescan::get_piece_mobility::<KNIGHT>(board, color, aux),
        bishop_mobility: movescan::get_piece_mobility::<BISHOP>(board, color, aux),
        rook_mobility: movescan::get_piece_mobility::<ROOK>(board, color, aux),
        queen_mobility: movescan::get_piece_mobility::<QUEEN>(board, color, aux),
    }
}

/// Gets coefficients of mobility for `board` and inserts them into `coeffs`. Similarly, their indices (starting from `index`) are inserted into `indices`.
/// Some additional data is also saved in `white_aux` and `black_aux` for further processing.
#[cfg(feature = "dev")]
pub fn get_coeffs(board: &Board, white_aux: &mut EvalAux, black_aux: &mut EvalAux, index: &mut u16, coeffs: &mut Vec<TunerCoeff>, indices: &mut Vec<u16>) {
    let white_data = get_mobility_data(board, WHITE, white_aux);
    let black_data = get_mobility_data(board, BLACK, black_aux);

    let mut data = [
        TunerCoeff::new(white_data.rook_open_file - black_data.rook_open_file, OPENING),
        TunerCoeff::new(white_data.rook_open_file - black_data.rook_open_file, ENDING),
        TunerCoeff::new(white_data.rook_semi_open_file - black_data.rook_semi_open_file, OPENING),
        TunerCoeff::new(white_data.rook_semi_open_file - black_data.rook_semi_open_file, ENDING),
        //
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
