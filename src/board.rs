use crate::{common::*, movescan::*};
pub struct Bitboard {
    pub pieces: [[u64; 6]; 2],
    pub occupancy: [u64; 2],
    pub color_to_move: Color,
    pub captured_pieces: Vec<u8>,
    pub piece_table: [u8; 64],
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard {
            pieces: [
                [
                    0x000000000000ff00,
                    0x0000000000000042,
                    0x0000000000000024,
                    0x0000000000000081,
                    0x0000000000000010,
                    0x0000000000000008,
                ],
                [
                    0x00ff000000000000,
                    0x4200000000000000,
                    0x2400000000000000,
                    0x8100000000000000,
                    0x1000000000000000,
                    0x0800000000000000,
                ],
            ],
            occupancy: [0xffff, 0xffff000000000000],
            color_to_move: WHITE,
            captured_pieces: Vec::with_capacity(16),

            #[rustfmt::skip]
            piece_table: [
                3, 1, 2, 5, 4, 2, 1, 3,
                0, 0, 0, 0, 0, 0, 0, 0,
                128, 128, 128, 128, 128, 128, 128, 128,
                128, 128, 128, 128, 128, 128, 128, 128,
                128, 128, 128, 128, 128, 128, 128, 128,
                128, 128, 128, 128, 128, 128, 128, 128,
                0, 0, 0, 0, 0, 0, 0, 0,
                3, 1, 2, 5, 4, 2, 1, 3,
            ],
        }
    }

    pub fn get_moves<const color: u8>(&self, mut moves: &mut [Move]) -> usize {
        let mut index = 0;
        index = scan_pawn_moves::<color>(&self, &mut moves, index);
        index = scan_piece_moves::<color, KNIGHT>(&self, &mut moves, index);
        index = scan_piece_moves::<color, BISHOP>(&self, &mut moves, index);
        index = scan_piece_moves::<color, ROOK>(&self, &mut moves, index);
        index = scan_piece_moves::<color, QUEEN>(&self, &mut moves, index);
        index = scan_piece_moves::<color, KING>(&self, &mut moves, index);

        index
    }

    pub fn make_move(&mut self, r#move: &Move) {
        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(from, self.color_to_move);

        let enemy_color = match self.color_to_move {
            WHITE => BLACK,
            BLACK => WHITE,
            _ => u8::MAX,
        };

        match flags {
            MoveFlags::Quiet => {
                self.move_piece(from, to, piece, self.color_to_move);
            }
            MoveFlags::DoublePush => {
                self.move_piece(from, to, piece, self.color_to_move);
            }
            MoveFlags::Capture => {
                let captured_piece = self.get_piece(to, enemy_color);
                self.captured_pieces.push(captured_piece);

                self.remove_piece(to, captured_piece, enemy_color);
                self.move_piece(from, to, piece, self.color_to_move);
            }
            _ => {}
        }

        self.color_to_move = enemy_color;
    }

    pub fn undo_move(&mut self, r#move: &Move) {
        let old_Color = self.color_to_move;
        let enemy_color = match self.color_to_move {
            WHITE => BLACK,
            BLACK => WHITE,
            _ => u8::MAX,
        };
        self.color_to_move = enemy_color;

        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(to, self.color_to_move);

        match flags {
            MoveFlags::Quiet => {
                self.move_piece(to, from, piece, self.color_to_move);
            }
            MoveFlags::DoublePush => {
                self.move_piece(to, from, piece, self.color_to_move);
            }
            MoveFlags::Capture => {
                let captured_piece = self.captured_pieces.pop().unwrap();

                self.move_piece(to, from, piece, self.color_to_move);
                self.add_piece(to, captured_piece, old_Color);
            }
            _ => {}
        }
    }

    fn get_piece(&self, field: u8, color: Color) -> u8 {
        self.piece_table[field as usize]
    }

    fn add_piece(&mut self, field: u8, piece: Piece, color: Color) {
        self.pieces[color as usize][piece as usize] |= 1u64 << field;
        self.occupancy[color as usize] |= 1u64 << field;
        self.piece_table[field as usize] = piece;
    }

    fn remove_piece(&mut self, field: u8, piece: Piece, color: Color) {
        self.pieces[color as usize][piece as usize] &= !(1u64 << field);
        self.occupancy[color as usize] &= !(1u64 << field);
        self.piece_table[field as usize] = 128;
    }

    fn move_piece(&mut self, from: u8, to: u8, piece: Piece, color: Color) {
        self.pieces[color as usize][piece as usize] ^= (1u64 << from) | (1u64 << to);
        self.occupancy[color as usize] ^= (1u64 << from) | (1u64 << to);

        self.piece_table[to as usize] = self.piece_table[from as usize];
        self.piece_table[from as usize] = 128;
    }
}
