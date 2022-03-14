use super::*;

static mut FILE_PATTERNS: [u64; 64] = [0; 64];
static mut RANK_PATTERNS: [u64; 64] = [0; 64];
static mut DIAGONAL_PATTERNS: [u64; 64] = [0; 64];
static mut JUMP_PATTERNS: [u64; 64] = [0; 64];
static mut BOX_PATTERNS: [u64; 64] = [0; 64];
static mut RAIL_PATTERNS: [u64; 8] = [0; 8];
static mut STAR_PATTERNS: [u64; 64] = [0; 64];
static mut FRONT_PATTERNS: [[u64; 64]; 2] = [[0; 64]; 2];

/// Initializes all patterns by filling arrays.
pub fn init() {
    generate_files();
    generate_ranks();
    generate_diagonals();
    generate_jumps();
    generate_boxes();
    generate_rails();
    generate_stars();
    generate_fronts();
}

/// Gets a file pattern for the field specified by `field_index`.
/// ```
/// . . . x . . . .
/// . . . x . . . .
/// . . . x . . . .
/// . . . x . . . .
/// . . . o . . . .
/// . . . x . . . .
/// . . . x . . . .
/// . . . x . . . .
/// ```
pub fn get_file(field_index: usize) -> u64 {
    unsafe { FILE_PATTERNS[field_index] }
}

/// Gets a rank pattern for the field specified by `field_index`.
/// ```
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// x x x o x x x x
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// ```
pub fn get_rank(field_index: usize) -> u64 {
    unsafe { RANK_PATTERNS[field_index] }
}

/// Gets a diagonal pattern for the field specified by `field_index`.
/// ```
/// . . . . . . . x
/// x . . . . . x .
/// . x . . . x . .
/// . . x . x . . .
/// . . . o . . . .
/// . . x . x . . .
/// . x . . . x . .
/// x . . . . . x .
/// ```
pub fn get_diagonals(field_index: usize) -> u64 {
    unsafe { DIAGONAL_PATTERNS[field_index] }
}

/// Get a jumps pattern for the field specified by `field_index`.
/// ```
/// . . . . . . . .
/// . . . . . . . .
/// . . x . x . . .
/// . x . . . x . .
/// . . . o . . . .
/// . x . . . x . .
/// . . x . x . . .
/// . . . . . . . .
/// ```
pub fn get_jumps(field_index: usize) -> u64 {
    unsafe { JUMP_PATTERNS[field_index] }
}

/// Get a box pattern for the field specified by `field_index`.
/// ```
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// . . x x x . . .
/// . . x o x . . .
/// . . x x x . . .
/// . . . . . . . .
/// . . . . . . . .
/// ```
pub fn get_box(field_index: usize) -> u64 {
    unsafe { BOX_PATTERNS[field_index] }
}

/// Get a rail pattern for the field specified by `field_index`.
/// ```
/// . . x . x . . .
/// . . x . x . . .
/// . . x . x . . .
/// . . x . x . . .
/// . . x o x . . .
/// . . x . x . . .
/// . . x . x . . .
/// . . x . x . . .
/// ```
pub fn get_rail(file: usize) -> u64 {
    unsafe { RAIL_PATTERNS[file] }
}

/// Get a star pattern for the field specified by `field_index`.
/// ```
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// . . x . x . . .
/// . . . o . . . .
/// . . x . x . . .
/// . . . . . . . .
/// . . . . . . . .
/// ```
pub fn get_star(field_index: usize) -> u64 {
    unsafe { STAR_PATTERNS[field_index] }
}

/// Get a front pattern for the field specified by `field_index`, from the `color` perspective.
/// ```
/// . . x x x . . .
/// . . x x x . . .
/// . . x x x . . .
/// . . x x x . . .
/// . . . o . . . .
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// ```
pub fn get_front(color: usize, field_index: usize) -> u64 {
    unsafe { FRONT_PATTERNS[color][field_index] }
}

/// Generates file patterns for all fields.
fn generate_files() {
    for field_index in 0..64 {
        unsafe { FILE_PATTERNS[field_index] = (FILE_H << (field_index % 8)) & !(1u64 << field_index) };
    }
}

/// Generates rank patterns for all fields.
fn generate_ranks() {
    for field_index in 0..64 {
        unsafe { RANK_PATTERNS[field_index] = (RANK_A << (8 * (field_index / 8))) & !(1u64 << field_index) };
    }
}

/// Generates diagonal patterns for all fields.
fn generate_diagonals() {
    for field_index in 0..64 {
        let mut result = 0u64;

        for direction in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
            let mut current = ((field_index as isize) % 8 + direction.0, (field_index as isize) / 8 + direction.1);

            while current.0 >= 0 && current.0 <= 7 && current.1 >= 0 && current.1 <= 7 {
                result |= 1u64 << (current.0 + current.1 * 8);
                current = (current.0 + direction.0, current.1 + direction.1);
            }
        }

        unsafe { DIAGONAL_PATTERNS[field_index] = result };
    }
}

/// Generates jump patterns for all fiellds.
fn generate_jumps() {
    for field_index in 0..64 {
        let field = 1u64 << field_index;

        unsafe {
            JUMP_PATTERNS[field_index] = 0
                | ((field & !FILE_G & !FILE_H) << 6)
                | ((field & !FILE_A & !FILE_B) >> 6)
                | ((field & !FILE_A & !FILE_B) << 10)
                | ((field & !FILE_G & !FILE_H) >> 10)
                | ((field & !FILE_H) << 15)
                | ((field & !FILE_A) >> 15)
                | ((field & !FILE_A) << 17)
                | ((field & !FILE_H) >> 17)
        };
    }
}

/// Generates box patterns for all fields.
fn generate_boxes() {
    for field_index in 0..64 {
        let field = 1u64 << field_index;

        unsafe {
            BOX_PATTERNS[field_index] = 0
                | ((field & !FILE_A) << 1)
                | ((field & !FILE_H) >> 1)
                | ((field & !FILE_H) << 7)
                | ((field & !FILE_A) >> 7)
                | ((field & !RANK_H) << 8)
                | ((field & !RANK_A) >> 8)
                | ((field & !FILE_A) << 9)
                | ((field & !FILE_H) >> 9)
        };
    }
}

/// Generates rail patterns for all fields.
fn generate_rails() {
    for file in 0..8 {
        let left_file = if file > 0 { FILE_H << (file - 1) } else { 0 };
        let right_file = if file < 7 { FILE_H << (file + 1) } else { 0 };
        unsafe { RAIL_PATTERNS[file] = left_file | right_file };
    }
}

/// Generates star patterns for all fields.
fn generate_stars() {
    for field_index in 0..64 {
        unsafe { STAR_PATTERNS[field_index] = DIAGONAL_PATTERNS[field_index] & BOX_PATTERNS[field_index] };
    }
}

/// Generates front patterns for all fields.
fn generate_fronts() {
    for color in 0..2 {
        for field_index in 0..64 {
            let file = field_index % 8;
            let rank = field_index / 8;

            let center_file = FILE_H << file;
            let left_file = if file > 0 { FILE_H << (file - 1) } else { 0 };
            let right_file = if file < 7 { FILE_H << (file + 1) } else { 0 };

            let mut current_rank = rank;
            let mut forbidden_area = 0;
            while (0..8).contains(&current_rank) {
                forbidden_area |= 255 << (current_rank * 8);
                current_rank += (color as i8) * 2 - 1;
            }

            unsafe { FRONT_PATTERNS[color][field_index as usize] = (left_file | center_file | right_file) & !forbidden_area };
        }
    }
}
