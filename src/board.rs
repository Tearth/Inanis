use crate::{common::*, movescan::*};

pub struct Bitboard {
    pub pieces: [[u64; 6]; 2],
    pub occupancy: [u64; 2],
    pub piece_table: [u8; 64],
    pub color_to_move: u8,
    pub captured_pieces: Vec<u8>,
}

impl Bitboard {
    pub fn new(without_init: bool) -> Bitboard {
        if without_init {
            return Bitboard {
                pieces: [[0; 6], [0; 6]],
                occupancy: [0, 2],
                piece_table: [0; 64],
                color_to_move: WHITE,
                captured_pieces: Vec::new(),
            };
        }

        Bitboard {
            #[rustfmt::skip]
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

            #[rustfmt::skip]
            occupancy: [
                0xffff,
                0xffff000000000000
            ],

            #[rustfmt::skip]
            piece_table: [
                3, 1, 2, 5, 4, 2, 1, 3,
                0, 0, 0, 0, 0, 0, 0, 0,
                u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX,
                u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX,
                u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX,
                u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX,
                0, 0, 0, 0, 0, 0, 0, 0,
                3, 1, 2, 5, 4, 2, 1, 3,
            ],

            color_to_move: WHITE,
            captured_pieces: Vec::with_capacity(16),
        }
    }

    pub fn get_moves<const COLOR: u8>(&self, mut moves: &mut [Move]) -> usize {
        let mut index = 0;
        index = scan_pawn_moves::<COLOR>(&self, &mut moves, index);
        index = scan_piece_moves::<COLOR, KNIGHT>(&self, &mut moves, index);
        index = scan_piece_moves::<COLOR, BISHOP>(&self, &mut moves, index);
        index = scan_piece_moves::<COLOR, ROOK>(&self, &mut moves, index);
        index = scan_piece_moves::<COLOR, QUEEN>(&self, &mut moves, index);
        index = scan_piece_moves::<COLOR, KING>(&self, &mut moves, index);

        index
    }

    pub fn make_move(&mut self, r#move: &Move) {
        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(from);
        let enemy_color = self.color_to_move ^ 1;

        match flags {
            MoveFlags::QUIET => {
                self.move_piece(from, to, piece, self.color_to_move);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(from, to, piece, self.color_to_move);
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.get_piece(to);
                self.captured_pieces.push(captured_piece);

                self.remove_piece(to, captured_piece, enemy_color);
                self.move_piece(from, to, piece, self.color_to_move);
            }
            _ => panic!("Invalid value: flags={:?}", flags),
        }

        self.color_to_move = enemy_color;
    }

    pub fn undo_move(&mut self, r#move: &Move) {
        let enemy_color = self.color_to_move;
        self.color_to_move ^= 1;

        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(to);

        match flags {
            MoveFlags::QUIET => {
                self.move_piece(to, from, piece, self.color_to_move);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(to, from, piece, self.color_to_move);
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.captured_pieces.pop().unwrap();

                self.move_piece(to, from, piece, self.color_to_move);
                self.add_piece(to, captured_piece, enemy_color);
            }
            _ => panic!("Invalid value: flags={:?}", flags),
        }
    }

    fn get_piece(&self, field: u8) -> u8 {
        self.piece_table[field as usize]
    }

    fn add_piece(&mut self, field: u8, piece: u8, color: u8) {
        self.pieces[color as usize][piece as usize] |= 1u64 << field;
        self.occupancy[color as usize] |= 1u64 << field;
        self.piece_table[field as usize] = piece;
    }

    fn remove_piece(&mut self, field: u8, piece: u8, color: u8) {
        self.pieces[color as usize][piece as usize] &= !(1u64 << field);
        self.occupancy[color as usize] &= !(1u64 << field);
        self.piece_table[field as usize] = u8::MAX;
    }

    fn move_piece(&mut self, from: u8, to: u8, piece: u8, color: u8) {
        self.pieces[color as usize][piece as usize] ^= (1u64 << from) | (1u64 << to);
        self.occupancy[color as usize] ^= (1u64 << from) | (1u64 << to);

        self.piece_table[to as usize] = self.piece_table[from as usize];
        self.piece_table[from as usize] = u8::MAX;
    }
}
