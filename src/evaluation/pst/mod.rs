use crate::state::board::Bitboard;
use crate::state::*;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

pub static mut TABLE: [[[[i8; 64]; 2]; 2]; 6] = [[[[0; 64]; 2]; 2]; 6];

pub fn init() {
    unsafe {
        for color in [WHITE, BLACK] {
            for phase in [OPENING, ENDING] {
                TABLE[PAWN as usize][color as usize][phase as usize] = calculate_pattern(color, &pawn::PATTERN[phase as usize]);
                TABLE[KNIGHT as usize][color as usize][phase as usize] = calculate_pattern(color, &knight::PATTERN[phase as usize]);
                TABLE[BISHOP as usize][color as usize][phase as usize] = calculate_pattern(color, &bishop::PATTERN[phase as usize]);
                TABLE[ROOK as usize][color as usize][phase as usize] = calculate_pattern(color, &rook::PATTERN[phase as usize]);
                TABLE[QUEEN as usize][color as usize][phase as usize] = calculate_pattern(color, &queen::PATTERN[phase as usize]);
                TABLE[KING as usize][color as usize][phase as usize] = calculate_pattern(color, &king::PATTERN[phase as usize]);
            }
        }
    }
}

pub fn evaluate(board: &Bitboard) -> i16 {
    let initial_material = 7920;
    let total_material = board.material_scores[WHITE as usize] + board.material_scores[BLACK as usize] - 20000;
    let game_phase = (total_material as f32) / (initial_material as f32);

    let opening_score = board.pst_scores[WHITE as usize][OPENING as usize] - board.pst_scores[BLACK as usize][OPENING as usize];
    let ending_score = board.pst_scores[WHITE as usize][ENDING as usize] - board.pst_scores[BLACK as usize][ENDING as usize];

    (((opening_score as f32) * game_phase) + ((1.0 - game_phase) * (ending_score as f32))) as i16
}

pub fn recalculate_incremental_values(board: &mut Bitboard) {
    for color_index in 0..=1 {
        for phase in [OPENING, ENDING] {
            let mut score = 0;
            for piece_index in 0..=5 {
                let mut pieces = board.pieces[color_index][piece_index];
                while pieces != 0 {
                    let field = get_lsb(pieces);
                    let field_index = bit_scan(field);
                    pieces = pop_lsb(pieces);

                    score += unsafe { TABLE[piece_index as usize][color_index as usize][phase as usize][field_index as usize] as i16 };
                }
            }

            board.pst_scores[color_index][phase as usize] = score;
        }
    }
}

fn calculate_pattern(color: u8, pattern: &[i8; 64]) -> [i8; 64] {
    let mut array = [0; 64];

    match color {
        WHITE => {
            for field_index in 0..64 {
                array[field_index] = pattern[63 - field_index];
            }
        }
        BLACK => {
            for file in 0..8 {
                for rank in 0..8 {
                    array[file + rank * 8] = pattern[(7 - file) + rank * 8];
                }
            }
        }
        _ => panic!("Invalid value: color={}", color),
    }

    array
}
