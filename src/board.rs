use crate::bit::*;
use crate::common::*;
use crate::fen;
use crate::movegen;
use crate::movescan::{self, Move, MoveFlags};

bitflags! {
    pub struct CastlingRights: u8 {
        const NONE = 0;
        const WHITE_SHORT_CASTLING = 1;
        const WHITE_LONG_CASTLING = 2;
        const BLACK_SHORT_CASTLING = 4;
        const BLACK_LONG_CASTLING = 8;

        const WHITE_CASTLING = Self::WHITE_SHORT_CASTLING.bits | Self::WHITE_LONG_CASTLING.bits;
        const BLACK_CASTLING = Self::BLACK_SHORT_CASTLING.bits | Self::BLACK_LONG_CASTLING.bits;
        const ALL = Self::WHITE_CASTLING.bits | Self::BLACK_CASTLING.bits;
    }
}

pub struct Bitboard {
    pub pieces: [[u64; 6]; 2],
    pub occupancy: [u64; 2],
    pub piece_table: [u8; 64],
    pub active_color: u8,
    pub castling_rights: CastlingRights,
    pub en_passant: u64,
    pub halfmove_clock: u16,
    pub fullmove_number: u32,
    pub halfmove_clocks_stack: Vec<u16>,
    pub captured_pieces_stack: Vec<u8>,
    pub castling_rights_stack: Vec<CastlingRights>,
    pub en_passant_stack: Vec<u64>,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard {
            pieces: [[0; 6], [0; 6]],
            occupancy: [0, 0],
            piece_table: [u8::MAX; 64],
            active_color: WHITE,
            castling_rights: CastlingRights::NONE,
            en_passant: 0,
            halfmove_clock: 0,
            fullmove_number: 0,
            halfmove_clocks_stack: Vec::with_capacity(16),
            captured_pieces_stack: Vec::with_capacity(16),
            castling_rights_stack: Vec::with_capacity(16),
            en_passant_stack: Vec::with_capacity(16),
        }
    }

    pub fn new_default() -> Bitboard {
        Bitboard::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn new_from_fen(fen: &str) -> Result<Bitboard, &str> {
        fen::fen_to_board(fen)
    }

    pub fn get_moves<const COLOR: u8>(&self, moves: &mut [Move]) -> usize {
        self.get_moves_internal::<COLOR>(moves)
    }

    pub fn get_moves_active_color(&self, moves: &mut [Move]) -> usize {
        match self.active_color {
            WHITE => self.get_moves_internal::<WHITE>(moves),
            BLACK => self.get_moves_internal::<BLACK>(moves),
            _ => panic!("Invalid value: self.active_color={}", self.active_color),
        }
    }

    pub fn make_move<const COLOR: u8, const ENEMY_COLOR: u8>(&mut self, r#move: &Move) {
        self.make_move_internal::<COLOR, ENEMY_COLOR>(r#move);
    }

    pub fn make_move_active_color(&mut self, r#move: &Move) {
        match self.active_color {
            WHITE => self.make_move_internal::<WHITE, BLACK>(r#move),
            BLACK => self.make_move_internal::<BLACK, WHITE>(r#move),
            _ => panic!("Invalid value: self.active_color={}", self.active_color),
        };
    }

    pub fn undo_move<const COLOR: u8, const ENEMY_COLOR: u8>(&mut self, r#move: &Move) {
        self.undo_move_internal::<COLOR, ENEMY_COLOR>(r#move);
    }

    pub fn undo_move_active_color(&mut self, r#move: &Move) {
        match self.active_color {
            WHITE => self.undo_move_internal::<BLACK, WHITE>(r#move),
            BLACK => self.undo_move_internal::<WHITE, BLACK>(r#move),
            _ => panic!("Invalid value: self.active_color={}", self.active_color),
        };
    }

    pub fn is_field_attacked<const COLOR: u8>(&self, field_index: u8) -> bool {
        let enemy_color = COLOR ^ 1;
        let occupancy = self.occupancy[WHITE as usize] | self.occupancy[BLACK as usize];

        let rook_queen_attacks = movegen::get_rook_moves(occupancy, field_index as usize);
        let enemy_rooks_queens = self.pieces[enemy_color as usize][ROOK as usize] | self.pieces[enemy_color as usize][QUEEN as usize];
        if (rook_queen_attacks & enemy_rooks_queens) != 0 {
            return true;
        }

        let bishop_queen_attacks = movegen::get_bishop_moves(occupancy, field_index as usize);
        let enemy_bishops_queens = self.pieces[enemy_color as usize][BISHOP as usize] | self.pieces[enemy_color as usize][QUEEN as usize];
        if (bishop_queen_attacks & enemy_bishops_queens) != 0 {
            return true;
        }

        let knight_attacks = movegen::get_knight_moves(field_index as usize);
        let enemy_knights = self.pieces[enemy_color as usize][KNIGHT as usize];
        if (knight_attacks & enemy_knights) != 0 {
            return true;
        }

        let king_attacks = movegen::get_king_moves(field_index as usize);
        let enemy_kings = self.pieces[enemy_color as usize][KING as usize];
        if (king_attacks & enemy_kings) != 0 {
            return true;
        }

        let field = 1u64 << field_index;
        let potential_enemy_pawns = king_attacks & self.pieces[enemy_color as usize][PAWN as usize];
        let attacking_enemy_pawns = match COLOR {
            WHITE => field & ((potential_enemy_pawns >> 7) | (potential_enemy_pawns >> 9)),
            BLACK => field & ((potential_enemy_pawns << 7) | (potential_enemy_pawns << 9)),
            _ => panic!("Invalid value: COLOR={}", COLOR),
        };

        if attacking_enemy_pawns != 0 {
            return true;
        }

        false
    }

    pub fn are_fields_attacked<const COLOR: u8>(&self, field_indexes: &[u8]) -> bool {
        for field_index in field_indexes {
            if self.is_field_attacked::<COLOR>(*field_index) {
                return true;
            }
        }

        false
    }

    pub fn is_king_checked<const COLOR: u8>(&self) -> bool {
        self.is_field_attacked::<COLOR>(bit_scan(self.pieces[COLOR as usize][KING as usize]))
    }

    pub fn get_piece(&self, field: u8) -> u8 {
        self.piece_table[field as usize]
    }

    pub fn add_piece<const COLOR: u8>(&mut self, field: u8, piece: u8) {
        self.pieces[COLOR as usize][piece as usize] |= 1u64 << field;
        self.occupancy[COLOR as usize] |= 1u64 << field;
        self.piece_table[field as usize] = piece;
    }

    pub fn remove_piece<const COLOR: u8>(&mut self, field: u8, piece: u8) {
        self.pieces[COLOR as usize][piece as usize] &= !(1u64 << field);
        self.occupancy[COLOR as usize] &= !(1u64 << field);
        self.piece_table[field as usize] = u8::MAX;
    }

    pub fn move_piece<const COLOR: u8>(&mut self, from: u8, to: u8, piece: u8) {
        self.pieces[COLOR as usize][piece as usize] ^= (1u64 << from) | (1u64 << to);
        self.occupancy[COLOR as usize] ^= (1u64 << from) | (1u64 << to);

        self.piece_table[to as usize] = self.piece_table[from as usize];
        self.piece_table[from as usize] = u8::MAX;
    }

    fn get_moves_internal<const COLOR: u8>(&self, mut moves: &mut [Move]) -> usize {
        let mut index = 0;
        index = movescan::scan_pawn_moves::<COLOR>(&self, &mut moves, index);
        index = movescan::scan_piece_moves::<COLOR, KNIGHT>(&self, &mut moves, index);
        index = movescan::scan_piece_moves::<COLOR, BISHOP>(&self, &mut moves, index);
        index = movescan::scan_piece_moves::<COLOR, ROOK>(&self, &mut moves, index);
        index = movescan::scan_piece_moves::<COLOR, QUEEN>(&self, &mut moves, index);
        index = movescan::scan_piece_moves::<COLOR, KING>(&self, &mut moves, index);

        index
    }

    fn make_move_internal<const COLOR: u8, const ENEMY_COLOR: u8>(&mut self, r#move: &Move) {
        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(from);

        self.halfmove_clocks_stack.push(self.halfmove_clock);
        self.castling_rights_stack.push(self.castling_rights);
        self.en_passant_stack.push(self.en_passant);
        self.en_passant = 0;

        match flags {
            MoveFlags::QUIET => {
                self.move_piece::<COLOR>(from, to, piece);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece::<COLOR>(from, to, piece);
                self.en_passant = 1u64 << ((to as i8) + 8 * ((COLOR as i8) * 2 - 1));
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.get_piece(to);
                self.captured_pieces_stack.push(captured_piece);

                self.remove_piece::<ENEMY_COLOR>(to, captured_piece);
                self.move_piece::<COLOR>(from, to, piece);
            }
            MoveFlags::SHORT_CASTLING => {
                self.move_piece::<COLOR>(3 + 56 * (COLOR as u8), 1 + 56 * (COLOR as u8), KING);
                self.move_piece::<COLOR>(0 + 56 * (COLOR as u8), 2 + 56 * (COLOR as u8), ROOK);
            }
            MoveFlags::LONG_CASTLING => {
                self.move_piece::<COLOR>(3 + 56 * (COLOR as u8), 5 + 56 * (COLOR as u8), KING);
                self.move_piece::<COLOR>(7 + 56 * (COLOR as u8), 4 + 56 * (COLOR as u8), ROOK);
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece::<COLOR>(from, to, piece);
                self.remove_piece::<ENEMY_COLOR>(((to as i8) + 8 * ((COLOR as i8) * 2 - 1)) as u8, PAWN);
            }
            _ => {
                // Promotion bit set, coincidentally knight promotion is the same
                if flags.contains(MoveFlags::KNIGHT_PROMOTION) {
                    let promotion_piece = r#move.get_promotion_piece();

                    if flags.contains(MoveFlags::CAPTURE) {
                        let captured_piece = self.get_piece(to);
                        self.captured_pieces_stack.push(captured_piece);

                        self.remove_piece::<ENEMY_COLOR>(to, captured_piece);
                    }

                    self.remove_piece::<COLOR>(from, PAWN);
                    self.add_piece::<COLOR>(to, promotion_piece);
                }
            }
        }

        if piece == KING {
            match COLOR {
                WHITE => {
                    self.castling_rights &= !CastlingRights::WHITE_CASTLING;
                }
                BLACK => {
                    self.castling_rights &= !CastlingRights::BLACK_CASTLING;
                }
                _ => panic!("Invalid value: COLOR={}", COLOR),
            }
        }

        if piece == ROOK {
            match COLOR {
                WHITE => {
                    if from == 0 {
                        self.castling_rights &= !CastlingRights::WHITE_SHORT_CASTLING;
                    } else if from == 7 {
                        self.castling_rights &= !CastlingRights::WHITE_LONG_CASTLING;
                    }
                }
                BLACK => {
                    if from == 56 {
                        self.castling_rights &= !CastlingRights::BLACK_SHORT_CASTLING;
                    } else if from == 63 {
                        self.castling_rights &= !CastlingRights::BLACK_LONG_CASTLING;
                    }
                }
                _ => panic!("Invalid value: COLOR={}", COLOR),
            }
        }

        if COLOR == BLACK {
            self.fullmove_number += 1;
        }

        if piece == PAWN || flags.contains(MoveFlags::CAPTURE) {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.active_color = ENEMY_COLOR;
    }

    fn undo_move_internal<const COLOR: u8, const ENEMY_COLOR: u8>(&mut self, r#move: &Move) {
        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(to);

        match flags {
            MoveFlags::QUIET => {
                self.move_piece::<COLOR>(to, from, piece);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece::<COLOR>(to, from, piece);
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.captured_pieces_stack.pop().unwrap();

                self.move_piece::<COLOR>(to, from, piece);
                self.add_piece::<ENEMY_COLOR>(to, captured_piece);
            }
            MoveFlags::SHORT_CASTLING => {
                self.move_piece::<COLOR>(1 + 56 * (COLOR as u8), 3 + 56 * (COLOR as u8), KING);
                self.move_piece::<COLOR>(2 + 56 * (COLOR as u8), 0 + 56 * (COLOR as u8), ROOK);
            }
            MoveFlags::LONG_CASTLING => {
                self.move_piece::<COLOR>(5 + 56 * (COLOR as u8), 3 + 56 * (COLOR as u8), KING);
                self.move_piece::<COLOR>(4 + 56 * (COLOR as u8), 7 + 56 * (COLOR as u8), ROOK);
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece::<COLOR>(to, from, piece);
                self.add_piece::<ENEMY_COLOR>(((to as i8) + 8 * ((COLOR as i8) * 2 - 1)) as u8, PAWN);
            }
            _ => {
                // Promotion bit set, coincidentally knight promotion is the same
                if flags.contains(MoveFlags::KNIGHT_PROMOTION) {
                    self.add_piece::<COLOR>(from, PAWN);
                    self.remove_piece::<COLOR>(to, piece);

                    if flags.contains(MoveFlags::CAPTURE) {
                        let captured_piece = self.captured_pieces_stack.pop().unwrap();
                        self.add_piece::<ENEMY_COLOR>(to, captured_piece);
                    }
                }
            }
        }

        if COLOR == BLACK {
            self.fullmove_number -= 1;
        }

        self.halfmove_clock = self.halfmove_clocks_stack.pop().unwrap();
        self.castling_rights = self.castling_rights_stack.pop().unwrap();
        self.en_passant = self.en_passant_stack.pop().unwrap();
        self.active_color = COLOR;
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new()
    }
}
