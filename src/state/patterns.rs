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
    pub fn get_file(&self, field_index: usize) -> u64 {
        self.file_patterns[field_index]
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
    pub fn get_rank(&self, field_index: usize) -> u64 {
        self.rank_patterns[field_index]
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
    pub fn get_diagonals(&self, field_index: usize) -> u64 {
        self.diagonal_patterns[field_index]
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
    pub fn get_jumps(&self, field_index: usize) -> u64 {
        self.jump_patterns[field_index]
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
    pub fn get_box(&self, field_index: usize) -> u64 {
        self.box_patterns[field_index]
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
    pub fn get_rail(&self, file: usize) -> u64 {
        self.rail_patterns[file]
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
    pub fn get_star(&self, field_index: usize) -> u64 {
        self.star_patterns[field_index]
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
    pub fn get_front(&self, color: usize, field_index: usize) -> u64 {
        self.front_patterns[color][field_index]
    }

    /// Generates file patterns for all fields.
    pub fn regenerate_files(&mut self) {
        for field_index in 0..64 {
            self.file_patterns[field_index] = (FILE_H << (field_index % 8)) & !(1u64 << field_index);
        }
    }

    /// Generates rank patterns for all fields.
    pub fn regenerate_ranks(&mut self) {
        for field_index in 0..64 {
            self.rank_patterns[field_index] = (RANK_A << (8 * (field_index / 8))) & !(1u64 << field_index);
        }
    }

    /// Generates diagonal patterns for all fields.
    pub fn regenerate_diagonals(&mut self) {
        for field_index in 0..64 {
            let mut result = 0u64;

            for direction in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                let mut current = ((field_index as isize) % 8 + direction.0, (field_index as isize) / 8 + direction.1);

                while current.0 >= 0 && current.0 <= 7 && current.1 >= 0 && current.1 <= 7 {
                    result |= 1u64 << (current.0 + current.1 * 8);
                    current = (current.0 + direction.0, current.1 + direction.1);
                }
            }

            self.diagonal_patterns[field_index] = result;
        }
    }

    /// Generates jump patterns for all fiellds.
    pub fn regenerate_jumps(&mut self) {
        for field_index in 0..64 {
            let field = 1u64 << field_index;

            self.jump_patterns[field_index] = 0
                | ((field & !FILE_G & !FILE_H) << 6)
                | ((field & !FILE_A & !FILE_B) >> 6)
                | ((field & !FILE_A & !FILE_B) << 10)
                | ((field & !FILE_G & !FILE_H) >> 10)
                | ((field & !FILE_H) << 15)
                | ((field & !FILE_A) >> 15)
                | ((field & !FILE_A) << 17)
                | ((field & !FILE_H) >> 17);
        }
    }

    /// Generates box patterns for all fields.
    pub fn regenerate_boxes(&mut self) {
        for field_index in 0..64 {
            let field = 1u64 << field_index;

            self.box_patterns[field_index] = 0
                | ((field & !FILE_A) << 1)
                | ((field & !FILE_H) >> 1)
                | ((field & !FILE_H) << 7)
                | ((field & !FILE_A) >> 7)
                | ((field & !RANK_H) << 8)
                | ((field & !RANK_A) >> 8)
                | ((field & !FILE_A) << 9)
                | ((field & !FILE_H) >> 9);
        }
    }

    /// Generates rail patterns for all fields.
    pub fn regenerate_rails(&mut self) {
        for file in 0..8 {
            let left_file = if file > 0 { FILE_H << (file - 1) } else { 0 };
            let right_file = if file < 7 { FILE_H << (file + 1) } else { 0 };
            self.rail_patterns[file] = left_file | right_file;
        }
    }

    /// Generates star patterns for all fields.
    pub fn regenerate_stars(&mut self) {
        for field_index in 0..64 {
            self.star_patterns[field_index] = self.diagonal_patterns[field_index] & self.box_patterns[field_index];
        }
    }

    /// Generates front patterns for all fields.
    pub fn regenerate_fronts(&mut self) {
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

                self.front_patterns[color][field_index as usize] = (left_file | center_file | right_file) & !forbidden_area;
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
