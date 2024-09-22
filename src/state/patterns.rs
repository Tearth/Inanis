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
    /// Gets a file pattern for the square specified by `square`.
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
    pub fn get_file(&self, square: usize) -> u64 {
        debug_assert!(square < 64);
        self.file_patterns[square]
    }

    /// Gets a rank pattern for the square specified by `square`.
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
    pub fn get_rank(&self, square: usize) -> u64 {
        debug_assert!(square < 64);
        self.rank_patterns[square]
    }

    /// Gets a diagonal pattern for the square specified by `square_`.
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
    pub fn get_diagonals(&self, square: usize) -> u64 {
        debug_assert!(square < 64);
        self.diagonal_patterns[square]
    }

    /// Get a jumps pattern for the square specified by `square`.
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
    pub fn get_jumps(&self, square: usize) -> u64 {
        debug_assert!(square < 64);
        self.jump_patterns[square]
    }

    /// Get a box pattern for the square specified by `square`.
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
    pub fn get_box(&self, square: usize) -> u64 {
        debug_assert!(square < 64);
        self.box_patterns[square]
    }

    /// Get a rail pattern for the square specified by `file`.
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
        debug_assert!(file < 8);
        self.rail_patterns[file]
    }

    /// Get a star pattern for the square specified by `square`.
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
    pub fn get_star(&self, square: usize) -> u64 {
        debug_assert!(square < 64);
        self.star_patterns[square]
    }

    /// Get a front pattern for the square specified by `square`, from the `color` perspective.
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
    pub fn get_front(&self, color: usize, square: usize) -> u64 {
        debug_assert!(color < 2);
        debug_assert!(square < 64);

        self.front_patterns[color][square]
    }

    /// Generates file patterns for all squares.
    pub fn regenerate_files(&mut self) {
        for square in ALL_SQUARES {
            self.file_patterns[square] = (FILE_H_BB << (square % 8)) & !(1u64 << square);
        }
    }

    /// Generates rank patterns for all squares.
    pub fn regenerate_ranks(&mut self) {
        for square in ALL_SQUARES {
            self.rank_patterns[square] = (RANK_1_BB << (8 * (square / 8))) & !(1u64 << square);
        }
    }

    /// Generates diagonal patterns for all squares.
    pub fn regenerate_diagonals(&mut self) {
        for square in ALL_SQUARES {
            let mut result = 0u64;

            for direction in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                let mut current = ((square as isize) % 8 + direction.0, (square as isize) / 8 + direction.1);

                while current.0 >= 0 && current.0 <= 7 && current.1 >= 0 && current.1 <= 7 {
                    result |= 1u64 << (current.0 + current.1 * 8);
                    current = (current.0 + direction.0, current.1 + direction.1);
                }
            }

            self.diagonal_patterns[square] = result;
        }
    }

    /// Generates jump patterns for all fiellds.
    pub fn regenerate_jumps(&mut self) {
        for square in ALL_SQUARES {
            let square_bb = 1u64 << square;

            self.jump_patterns[square] = 0
                | ((square_bb & !FILE_G_BB & !FILE_H_BB) << 6)
                | ((square_bb & !FILE_A_BB & !FILE_B_BB) >> 6)
                | ((square_bb & !FILE_A_BB & !FILE_B_BB) << 10)
                | ((square_bb & !FILE_G_BB & !FILE_H_BB) >> 10)
                | ((square_bb & !FILE_H_BB) << 15)
                | ((square_bb & !FILE_A_BB) >> 15)
                | ((square_bb & !FILE_A_BB) << 17)
                | ((square_bb & !FILE_H_BB) >> 17);
        }
    }

    /// Generates box patterns for all squares.
    pub fn regenerate_boxes(&mut self) {
        for square in ALL_SQUARES {
            let square_bb = 1u64 << square;

            self.box_patterns[square] = 0
                | ((square_bb & !FILE_A_BB) << 1)
                | ((square_bb & !FILE_H_BB) >> 1)
                | ((square_bb & !FILE_H_BB) << 7)
                | ((square_bb & !FILE_A_BB) >> 7)
                | ((square_bb & !RANK_8_BB) << 8)
                | ((square_bb & !RANK_1_BB) >> 8)
                | ((square_bb & !FILE_A_BB) << 9)
                | ((square_bb & !FILE_H_BB) >> 9);
        }
    }

    /// Generates rail patterns for all squares.
    pub fn regenerate_rails(&mut self) {
        for file in ALL_FILES {
            let left_file_bb = if file > 0 { FILE_H_BB << (file - 1) } else { 0 };
            let right_file_bb = if file < 7 { FILE_H_BB << (file + 1) } else { 0 };
            self.rail_patterns[file] = left_file_bb | right_file_bb;
        }
    }

    /// Generates star patterns for all squares.
    pub fn regenerate_stars(&mut self) {
        for square in ALL_SQUARES {
            self.star_patterns[square] = self.diagonal_patterns[square] & self.box_patterns[square];
        }
    }

    /// Generates front patterns for all squares.
    pub fn regenerate_fronts(&mut self) {
        for color in ALL_PHASES {
            for square in ALL_SQUARES {
                let file = square % 8;
                let rank = square / 8;

                let center_file_bb = FILE_H_BB << file;
                let left_file_bb = if file > 0 { FILE_H_BB << (file - 1) } else { 0 };
                let right_file_bb = if file < 7 { FILE_H_BB << (file + 1) } else { 0 };

                let mut current_rank = rank as i8;
                let mut forbidden_area = 0;
                while ALL_RANKS.contains(&(current_rank as usize)) {
                    forbidden_area |= 255 << (current_rank * 8);
                    current_rank += (color as i8) * 2 - 1;
                }

                self.front_patterns[color][square] = (left_file_bb | center_file_bb | right_file_bb) & !forbidden_area;
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
