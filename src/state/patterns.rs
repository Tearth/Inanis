use super::*;
use crate::utils::assert_fast;

pub const DIAGONAL_PATTERNS: [u64; 64] = generate_diagonals();
pub const JUMP_PATTERNS: [u64; 64] = generate_jumps();
pub const BOX_PATTERNS: [u64; 64] = generate_boxes();
pub const RAIL_PATTERNS: [u64; 8] = generate_rails();
pub const FRONT_PATTERNS: [[u64; 64]; 2] = generate_fronts();

/// Gets a file pattern for the square specified by `square`.
/// ```
/// . . . x . . . .
/// . . . x . . . .
/// . . . x . . . .
/// . . . x . . . .
/// . . . x . . . .
/// . . . x . . . .
/// . . . x . . . .
/// . . . x . . . .
/// ```
pub fn get_file(square: usize) -> u64 {
    assert_fast!(square < 64);
    FILE_H_BB << (square % 8)
}

/// Gets a rank pattern for the square specified by `square`.
/// ```
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// x x x x x x x x
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// ```
pub fn get_rank(square: usize) -> u64 {
    assert_fast!(square < 64);
    RANK_1_BB << ((square / 8) * 8)
}

/// Gets a diagonal pattern for the square specified by `square_`.
/// ```
/// . . . . . . . x
/// x . . . . . x .
/// . x . . . x . .
/// . . x . x . . .
/// . . . x . . . .
/// . . x . x . . .
/// . x . . . x . .
/// x . . . . . x .
/// ```
pub fn get_diagonals(square: usize) -> u64 {
    assert_fast!(square < 64);
    DIAGONAL_PATTERNS[square]
}

/// Get a jumps pattern for the square specified by `square`.
/// ```
/// . . . . . . . .
/// . . . . . . . .
/// . . x . x . . .
/// . x . . . x . .
/// . . . . . . . .
/// . x . . . x . .
/// . . x . x . . .
/// . . . . . . . .
/// ```
pub fn get_jumps(square: usize) -> u64 {
    assert_fast!(square < 64);
    JUMP_PATTERNS[square]
}

/// Get a box pattern for the square specified by `square`.
/// ```
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// . . x x x . . .
/// . . x x x . . .
/// . . x x x . . .
/// . . . . . . . .
/// . . . . . . . .
/// ```
pub fn get_box(square: usize) -> u64 {
    assert_fast!(square < 64);
    BOX_PATTERNS[square]
}

/// Get a rail pattern for the square specified by `file`.
/// ```
/// . . x . x . . .
/// . . x . x . . .
/// . . x . x . . .
/// . . x . x . . .
/// . . x . x . . .
/// . . x . x . . .
/// . . x . x . . .
/// . . x . x . . .
/// ```
pub fn get_rail(file: usize) -> u64 {
    assert_fast!(file < 8);
    RAIL_PATTERNS[file]
}

/// Get a front pattern for the square specified by `square`, from the `color` perspective.
/// ```
/// . . x x x . . .
/// . . x x x . . .
/// . . x x x . . .
/// . . x x x . . .
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// ```
pub fn get_front(color: usize, square: usize) -> u64 {
    assert_fast!(color < 2);
    assert_fast!(square < 64);
    FRONT_PATTERNS[color][square]
}

/// Generates diagonal patterns for all squares.
pub const fn generate_diagonals() -> [u64; 64] {
    let mut square = 0;
    let mut result = [0; 64];

    while square < 64 {
        let mut index = 0;
        while index < 4 {
            let offset = ((index % 2) * 2 - 1, (index / 2) * 2 - 1);
            let mut current = ((square as isize) % 8 + offset.0, (square as isize) / 8 + offset.1);

            while current.0 >= 0 && current.0 <= 7 && current.1 >= 0 && current.1 <= 7 {
                result[square] |= 1u64 << (current.0 + current.1 * 8);
                current = (current.0 + offset.0, current.1 + offset.1);
            }

            index += 1;
        }

        result[square] |= 1u64 << square;
        square += 1;
    }

    result
}

/// Generates jump patterns for all fields.
pub const fn generate_jumps() -> [u64; 64] {
    let mut square = 0;
    let mut result = [0; 64];

    while square < 64 {
        let square_bb = 1u64 << square;
        result[square] = 0
            | ((square_bb & !FILE_G_BB & !FILE_H_BB) << 6)
            | ((square_bb & !FILE_A_BB & !FILE_B_BB) >> 6)
            | ((square_bb & !FILE_A_BB & !FILE_B_BB) << 10)
            | ((square_bb & !FILE_G_BB & !FILE_H_BB) >> 10)
            | ((square_bb & !FILE_H_BB) << 15)
            | ((square_bb & !FILE_A_BB) >> 15)
            | ((square_bb & !FILE_A_BB) << 17)
            | ((square_bb & !FILE_H_BB) >> 17);

        square += 1;
    }

    result
}

/// Generates box patterns for all squares.
pub const fn generate_boxes() -> [u64; 64] {
    let mut square = 0;
    let mut result = [0; 64];

    while square < 64 {
        let square_bb = 1u64 << square;
        result[square] = 0
            | ((square_bb & !FILE_A_BB) << 1)
            | ((square_bb & !FILE_H_BB) >> 1)
            | ((square_bb & !FILE_H_BB) << 7)
            | ((square_bb & !FILE_A_BB) >> 7)
            | ((square_bb & !RANK_8_BB) << 8)
            | ((square_bb & !RANK_1_BB) >> 8)
            | ((square_bb & !FILE_A_BB) << 9)
            | ((square_bb & !FILE_H_BB) >> 9);

        square += 1;
    }

    result
}

/// Generates rail patterns for all squares.
pub const fn generate_rails() -> [u64; 8] {
    let mut file = 0;
    let mut result = [0; 8];

    while file < 8 {
        let left_file_bb = if file > 0 { FILE_H_BB << (file - 1) } else { 0 };
        let right_file_bb = if file < 7 { FILE_H_BB << (file + 1) } else { 0 };

        result[file] = left_file_bb | right_file_bb;
        file += 1;
    }

    result
}

/// Generates front patterns for all squares.
pub const fn generate_fronts() -> [[u64; 64]; 2] {
    let mut color = 0;
    let mut result = [[0; 64]; 2];

    while color < 2 {
        let mut square = 0;
        while square < 64 {
            let file = square % 8;
            let rank = square / 8;

            let center_file_bb = FILE_H_BB << file;
            let left_file_bb = if file > 0 { FILE_H_BB << (file - 1) } else { 0 };
            let right_file_bb = if file < 7 { FILE_H_BB << (file + 1) } else { 0 };

            let mut current_rank = rank as i8;
            let mut forbidden_area = 0;
            while current_rank >= 0 && current_rank < 8 {
                forbidden_area |= 255 << (current_rank * 8);
                current_rank += (color as i8) * 2 - 1;
            }

            result[color][square] = (left_file_bb | center_file_bb | right_file_bb) & !forbidden_area;
            square += 1;
        }

        color += 1;
    }

    result
}
