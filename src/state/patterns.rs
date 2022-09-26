use super::*;

pub struct PatternsContainer {
    pub file_patterns: [u64; 64],
    pub rank_patterns: [u64; 64],
    pub diagonal_patterns: [u64; 64],
    pub jump_patterns: [u64; 64],
    pub box_patterns: [u64; 64],
    pub rail_patterns: [u64; 8],
    pub star_patterns: [u64; 64],
    pub front_patterns: [[u64; 64]; 2],
}

impl PatternsContainer {
    /// Gets a file pattern for the square specified by `square_index`.
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
    pub fn get_file(&self, square_index: usize) -> u64 {
        self.file_patterns[square_index]
    }

    /// Gets a rank pattern for the square specified by `square_index`.
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
    pub fn get_rank(&self, square_index: usize) -> u64 {
        self.rank_patterns[square_index]
    }

    /// Gets a diagonal pattern for the square specified by `square_index`.
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
    pub fn get_diagonals(&self, square_index: usize) -> u64 {
        self.diagonal_patterns[square_index]
    }

    /// Get a jumps pattern for the square specified by `square_index`.
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
    pub fn get_jumps(&self, square_index: usize) -> u64 {
        self.jump_patterns[square_index]
    }

    /// Get a box pattern for the square specified by `square_index`.
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
    pub fn get_box(&self, square_index: usize) -> u64 {
        self.box_patterns[square_index]
    }

    /// Get a rail pattern for the square specified by `square_index`.
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
    pub fn get_rail(&self, file: usize) -> u64 {
        self.rail_patterns[file]
    }

    /// Get a star pattern for the square specified by `square_index`.
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
    pub fn get_star(&self, square_index: usize) -> u64 {
        self.star_patterns[square_index]
    }

    /// Get a front pattern for the square specified by `square_index`, from the `color` perspective.
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
    pub fn get_front(&self, color: usize, square_index: usize) -> u64 {
        self.front_patterns[color][square_index]
    }

    /// Generates file patterns for all squares.
    pub fn regenerate_files(&mut self) {
        for square_index in 0..64 {
            self.file_patterns[square_index] = (FILE_H << (square_index % 8)) & !(1u64 << square_index);
        }
    }

    /// Generates rank patterns for all squares.
    pub fn regenerate_ranks(&mut self) {
        for square_index in 0..64 {
            self.rank_patterns[square_index] = (RANK_1 << (8 * (square_index / 8))) & !(1u64 << square_index);
        }
    }

    /// Generates diagonal patterns for all squares.
    pub fn regenerate_diagonals(&mut self) {
        for square_index in 0..64 {
            let mut result = 0u64;

            for direction in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                let mut current = ((square_index as isize) % 8 + direction.0, (square_index as isize) / 8 + direction.1);

                while current.0 >= 0 && current.0 <= 7 && current.1 >= 0 && current.1 <= 7 {
                    result |= 1u64 << (current.0 + current.1 * 8);
                    current = (current.0 + direction.0, current.1 + direction.1);
                }
            }

            self.diagonal_patterns[square_index] = result;
        }
    }

    /// Generates jump patterns for all fiellds.
    pub fn regenerate_jumps(&mut self) {
        for square_index in 0..64 {
            let square = 1u64 << square_index;

            self.jump_patterns[square_index] = 0
                | ((square & !FILE_G & !FILE_H) << 6)
                | ((square & !FILE_A & !FILE_B) >> 6)
                | ((square & !FILE_A & !FILE_B) << 10)
                | ((square & !FILE_G & !FILE_H) >> 10)
                | ((square & !FILE_H) << 15)
                | ((square & !FILE_A) >> 15)
                | ((square & !FILE_A) << 17)
                | ((square & !FILE_H) >> 17);
        }
    }

    /// Generates box patterns for all squares.
    pub fn regenerate_boxes(&mut self) {
        for square_index in 0..64 {
            let square = 1u64 << square_index;

            self.box_patterns[square_index] = 0
                | ((square & !FILE_A) << 1)
                | ((square & !FILE_H) >> 1)
                | ((square & !FILE_H) << 7)
                | ((square & !FILE_A) >> 7)
                | ((square & !RANK_8) << 8)
                | ((square & !RANK_1) >> 8)
                | ((square & !FILE_A) << 9)
                | ((square & !FILE_H) >> 9);
        }
    }

    /// Generates rail patterns for all squares.
    pub fn regenerate_rails(&mut self) {
        for file in 0..8 {
            let left_file = if file > 0 { FILE_H << (file - 1) } else { 0 };
            let right_file = if file < 7 { FILE_H << (file + 1) } else { 0 };
            self.rail_patterns[file] = left_file | right_file;
        }
    }

    /// Generates star patterns for all squares.
    pub fn regenerate_stars(&mut self) {
        for square_index in 0..64 {
            self.star_patterns[square_index] = self.diagonal_patterns[square_index] & self.box_patterns[square_index];
        }
    }

    /// Generates front patterns for all squares.
    pub fn regenerate_fronts(&mut self) {
        for color in 0..2 {
            for square_index in 0..64 {
                let file = square_index % 8;
                let rank = square_index / 8;

                let center_file = FILE_H << file;
                let left_file = if file > 0 { FILE_H << (file - 1) } else { 0 };
                let right_file = if file < 7 { FILE_H << (file + 1) } else { 0 };

                let mut current_rank = rank;
                let mut forbidden_area = 0;
                while (0..8).contains(&current_rank) {
                    forbidden_area |= 255 << (current_rank * 8);
                    current_rank += (color as i8) * 2 - 1;
                }

                self.front_patterns[color][square_index as usize] = (left_file | center_file | right_file) & !forbidden_area;
            }
        }
    }
}

impl Default for PatternsContainer {
    /// Constructs a default instance of [PatternsContainer] with initialized patterns.
    fn default() -> Self {
        let mut result = Self {
            file_patterns: [0; 64],
            rank_patterns: [0; 64],
            diagonal_patterns: [0; 64],
            jump_patterns: [0; 64],
            box_patterns: [0; 64],
            rail_patterns: [0; 8],
            star_patterns: [0; 64],
            front_patterns: [[0; 64]; 2],
        };

        result.regenerate_files();
        result.regenerate_ranks();
        result.regenerate_diagonals();
        result.regenerate_jumps();
        result.regenerate_boxes();
        result.regenerate_rails();
        result.regenerate_stars();
        result.regenerate_fronts();

        result
    }
}
