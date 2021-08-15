use super::common::*;
use super::fen;
use super::movegen;
use super::movescan;
use super::movescan::Move;
use super::movescan::MoveFlags;
use super::zobrist;
use crate::evaluation;
use crate::evaluation::material;

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

#[derive(Clone)]
pub struct Bitboard {
    pub pieces: [[u64; 6]; 2],
    pub occupancy: [u64; 2],
    pub piece_table: [u8; 64],
    pub castling_rights: CastlingRights,
    pub en_passant: u64,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
    pub active_color: u8,
    pub hash: u64,
    pub halfmove_clocks_stack: Vec<u16>,
    pub captured_pieces_stack: Vec<u8>,
    pub castling_rights_stack: Vec<CastlingRights>,
    pub en_passant_stack: Vec<u64>,
    pub hash_stack: Vec<u64>,
    pub material_scores: [i16; 2],
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard {
            pieces: [[0; 6], [0; 6]],
            occupancy: [0, 0],
            piece_table: [u8::MAX; 64],
            castling_rights: CastlingRights::NONE,
            en_passant: 0,
            halfmove_clock: 0,
            fullmove_number: 0,
            active_color: WHITE,
            hash: 0,
            halfmove_clocks_stack: Vec::with_capacity(32),
            captured_pieces_stack: Vec::with_capacity(32),
            castling_rights_stack: Vec::with_capacity(32),
            en_passant_stack: Vec::with_capacity(32),
            hash_stack: Vec::with_capacity(32),
            material_scores: [0, 0],
        }
    }

    pub fn new_default() -> Bitboard {
        Bitboard::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn new_from_fen(fen: &str) -> Result<Bitboard, &'static str> {
        fen::fen_to_board(fen)
    }

    pub fn new_from_moves(moves: &[&str]) -> Result<Bitboard, &'static str> {
        let mut board = Bitboard::new_default();
        for premade_move in moves {
            let parsed_move = Move::from_text(premade_move.trim(), &board)?;
            board.make_move_active_color(&parsed_move);
        }

        Ok(board)
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

    pub fn make_move<const COLOR: u8>(&mut self, r#move: &Move) {
        self.make_move_internal::<COLOR>(r#move);
    }

    pub fn make_move_active_color(&mut self, r#move: &Move) {
        match self.active_color {
            WHITE => self.make_move_internal::<WHITE>(r#move),
            BLACK => self.make_move_internal::<BLACK>(r#move),
            _ => panic!("Invalid value: self.active_color={}", self.active_color),
        };
    }

    pub fn undo_move<const COLOR: u8>(&mut self, r#move: &Move) {
        self.undo_move_internal::<COLOR>(r#move);
    }

    pub fn undo_move_active_color(&mut self, r#move: &Move) {
        match self.active_color {
            WHITE => self.undo_move_internal::<BLACK>(r#move),
            BLACK => self.undo_move_internal::<WHITE>(r#move),
            _ => panic!("Invalid value: self.active_color={}", self.active_color),
        };
    }

    pub fn is_field_attacked(&self, color: u8, field_index: u8) -> bool {
        let enemy_color = color ^ 1;
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
        let attacking_enemy_pawns = match color {
            WHITE => field & ((potential_enemy_pawns >> 7) | (potential_enemy_pawns >> 9)),
            BLACK => field & ((potential_enemy_pawns << 7) | (potential_enemy_pawns << 9)),
            _ => panic!("Invalid value: color={}", color),
        };

        if attacking_enemy_pawns != 0 {
            return true;
        }

        false
    }

    pub fn are_fields_attacked(&self, color: u8, field_indexes: &[u8]) -> bool {
        for field_index in field_indexes {
            if self.is_field_attacked(color, *field_index) {
                return true;
            }
        }

        false
    }

    pub fn is_king_checked(&self, color: u8) -> bool {
        self.is_field_attacked(color, bit_scan(self.pieces[color as usize][KING as usize]))
    }

    pub fn get_piece(&self, field: u8) -> u8 {
        self.piece_table[field as usize]
    }

    pub fn add_piece(&mut self, color: u8, piece: u8, field: u8) {
        self.pieces[color as usize][piece as usize] |= 1u64 << field;
        self.occupancy[color as usize] |= 1u64 << field;
        self.piece_table[field as usize] = piece;
        self.material_scores[color as usize] += material::PIECE_VALUE[piece as usize];
    }

    pub fn remove_piece(&mut self, color: u8, piece: u8, field: u8) {
        self.pieces[color as usize][piece as usize] &= !(1u64 << field);
        self.occupancy[color as usize] &= !(1u64 << field);
        self.piece_table[field as usize] = u8::MAX;
        self.material_scores[color as usize] -= material::PIECE_VALUE[piece as usize];
    }

    pub fn move_piece(&mut self, color: u8, piece: u8, from: u8, to: u8) {
        self.pieces[color as usize][piece as usize] ^= (1u64 << from) | (1u64 << to);
        self.occupancy[color as usize] ^= (1u64 << from) | (1u64 << to);

        self.piece_table[to as usize] = self.piece_table[from as usize];
        self.piece_table[from as usize] = u8::MAX;
    }

    pub fn to_fen(&self) -> String {
        fen::board_to_fen(self)
    }

    pub fn recalculate_hash(&mut self) {
        zobrist::recalculate_hash(self);
    }

    pub fn evaluate(&self) -> i16 {
        material::evaluate(self)
    }

    pub fn recalculate_incremental_values(&mut self) {
        material::recalculate_incremental_values(self);
    }

    pub fn is_threefold_repetition_draw(&self) -> bool {
        let mut repetitions_count = 1;
        for hash in &self.hash_stack {
            if *hash == self.hash {
                repetitions_count += 1;
            }
        }

        repetitions_count >= 3
    }

    pub fn is_fifty_move_rule_draw(&self) -> bool {
        self.halfmove_clock >= 100
    }

    fn get_moves_internal<const COLOR: u8>(&self, mut moves: &mut [Move]) -> usize {
        let mut index = 0;
        index = movescan::scan_pawn_moves::<COLOR>(self, &mut moves, index);
        index = movescan::scan_piece_moves::<COLOR, KNIGHT>(self, &mut moves, index);
        index = movescan::scan_piece_moves::<COLOR, BISHOP>(self, &mut moves, index);
        index = movescan::scan_piece_moves::<COLOR, ROOK>(self, &mut moves, index);
        index = movescan::scan_piece_moves::<COLOR, QUEEN>(self, &mut moves, index);
        index = movescan::scan_piece_moves::<COLOR, KING>(self, &mut moves, index);

        index
    }

    fn make_move_internal<const COLOR: u8>(&mut self, r#move: &Move) {
        let enemy_color = COLOR ^ 1;

        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(from);

        self.halfmove_clocks_stack.push(self.halfmove_clock);
        self.castling_rights_stack.push(self.castling_rights);
        self.en_passant_stack.push(self.en_passant);
        self.hash_stack.push(self.hash);

        if self.en_passant != 0 {
            zobrist::toggle_en_passant(&mut self.hash, (bit_scan(self.en_passant) % 8) as u8);
            self.en_passant = 0;
        }

        match flags {
            MoveFlags::QUIET => {
                self.move_piece(COLOR, piece, from, to);
                zobrist::toggle_piece(&mut self.hash, COLOR, piece, from);
                zobrist::toggle_piece(&mut self.hash, COLOR, piece, to);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(COLOR, piece, from, to);
                zobrist::toggle_piece(&mut self.hash, COLOR, piece, from);
                zobrist::toggle_piece(&mut self.hash, COLOR, piece, to);

                self.en_passant = 1u64 << ((to as i8) + 8 * ((COLOR as i8) * 2 - 1));
                zobrist::toggle_en_passant(&mut self.hash, (bit_scan(self.en_passant) % 8) as u8);
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.get_piece(to);
                self.captured_pieces_stack.push(captured_piece);

                self.remove_piece(enemy_color, captured_piece, to);
                zobrist::toggle_piece(&mut self.hash, enemy_color, captured_piece, to);

                self.move_piece(COLOR, piece, from, to);
                zobrist::toggle_piece(&mut self.hash, COLOR, piece, from);
                zobrist::toggle_piece(&mut self.hash, COLOR, piece, to);
            }
            MoveFlags::SHORT_CASTLING => {
                let king_from = 3 + 56 * (COLOR as u8);
                let king_to = 1 + 56 * (COLOR as u8);

                self.move_piece(COLOR, KING, king_from, king_to);
                zobrist::toggle_piece(&mut self.hash, COLOR, KING, king_from);
                zobrist::toggle_piece(&mut self.hash, COLOR, KING, king_to);

                let rook_from = 0 + 56 * (COLOR as u8);
                let rook_to = 2 + 56 * (COLOR as u8);

                self.move_piece(COLOR, ROOK, rook_from, rook_to);
                zobrist::toggle_piece(&mut self.hash, COLOR, ROOK, rook_from);
                zobrist::toggle_piece(&mut self.hash, COLOR, ROOK, rook_to);
            }
            MoveFlags::LONG_CASTLING => {
                let king_from = 3 + 56 * (COLOR as u8);
                let king_to = 5 + 56 * (COLOR as u8);

                self.move_piece(COLOR, KING, 3 + 56 * (COLOR as u8), 5 + 56 * (COLOR as u8));
                zobrist::toggle_piece(&mut self.hash, COLOR, KING, king_from);
                zobrist::toggle_piece(&mut self.hash, COLOR, KING, king_to);

                let rook_from = 7 + 56 * (COLOR as u8);
                let rook_to = 4 + 56 * (COLOR as u8);

                self.move_piece(COLOR, ROOK, 7 + 56 * (COLOR as u8), 4 + 56 * (COLOR as u8));
                zobrist::toggle_piece(&mut self.hash, COLOR, ROOK, rook_from);
                zobrist::toggle_piece(&mut self.hash, COLOR, ROOK, rook_to);
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece(COLOR, piece, from, to);
                zobrist::toggle_piece(&mut self.hash, COLOR, piece, from);
                zobrist::toggle_piece(&mut self.hash, COLOR, piece, to);

                let enemy_pawn_field_index = ((to as i8) + 8 * ((COLOR as i8) * 2 - 1)) as u8;

                self.remove_piece(enemy_color, PAWN, enemy_pawn_field_index);
                zobrist::toggle_piece(&mut self.hash, enemy_color, PAWN, enemy_pawn_field_index);
            }
            _ => {
                let promotion_piece = r#move.get_promotion_piece();
                if flags.contains(MoveFlags::CAPTURE) {
                    let captured_piece = self.get_piece(to);
                    self.captured_pieces_stack.push(captured_piece);

                    self.remove_piece(enemy_color, captured_piece, to);
                    zobrist::toggle_piece(&mut self.hash, enemy_color, captured_piece, to);
                }

                self.remove_piece(COLOR, PAWN, from);
                zobrist::toggle_piece(&mut self.hash, COLOR, PAWN, from);

                self.add_piece(COLOR, promotion_piece, to);
                zobrist::toggle_piece(&mut self.hash, COLOR, promotion_piece, to);
            }
        }

        if piece == KING {
            self.castling_rights &= match COLOR {
                WHITE => {
                    zobrist::toggle_castling_right(&mut self.hash, self.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                    zobrist::toggle_castling_right(&mut self.hash, self.castling_rights, CastlingRights::WHITE_LONG_CASTLING);

                    !CastlingRights::WHITE_CASTLING
                }
                BLACK => {
                    zobrist::toggle_castling_right(&mut self.hash, self.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                    zobrist::toggle_castling_right(&mut self.hash, self.castling_rights, CastlingRights::BLACK_LONG_CASTLING);

                    !CastlingRights::BLACK_CASTLING
                }
                _ => panic!("Invalid value: COLOR={}", COLOR),
            }
        }

        if piece == ROOK {
            match COLOR {
                WHITE => {
                    if from == 0 {
                        zobrist::toggle_castling_right(&mut self.hash, self.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                        self.castling_rights &= !CastlingRights::WHITE_SHORT_CASTLING;
                    } else if from == 7 {
                        zobrist::toggle_castling_right(&mut self.hash, self.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
                        self.castling_rights &= !CastlingRights::WHITE_LONG_CASTLING;
                    }
                }
                BLACK => {
                    if from == 56 {
                        zobrist::toggle_castling_right(&mut self.hash, self.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                        self.castling_rights &= !CastlingRights::BLACK_SHORT_CASTLING;
                    } else if from == 63 {
                        zobrist::toggle_castling_right(&mut self.hash, self.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
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

        self.active_color = enemy_color;
        zobrist::toggle_active_color(&mut self.hash);
    }

    fn undo_move_internal<const COLOR: u8>(&mut self, r#move: &Move) {
        let enemy_color = COLOR ^ 1;

        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(to);

        self.halfmove_clock = self.halfmove_clocks_stack.pop().unwrap();
        self.castling_rights = self.castling_rights_stack.pop().unwrap();
        self.en_passant = self.en_passant_stack.pop().unwrap();
        self.hash = self.hash_stack.pop().unwrap();

        match flags {
            MoveFlags::QUIET => {
                self.move_piece(COLOR, piece, to, from);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(COLOR, piece, to, from);
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.captured_pieces_stack.pop().unwrap();

                self.move_piece(COLOR, piece, to, from);
                self.add_piece(enemy_color, captured_piece, to);
            }
            MoveFlags::SHORT_CASTLING => {
                self.move_piece(COLOR, KING, 1 + 56 * (COLOR as u8), 3 + 56 * (COLOR as u8));
                self.move_piece(COLOR, ROOK, 2 + 56 * (COLOR as u8), 0 + 56 * (COLOR as u8));
            }
            MoveFlags::LONG_CASTLING => {
                self.move_piece(COLOR, KING, 5 + 56 * (COLOR as u8), 3 + 56 * (COLOR as u8));
                self.move_piece(COLOR, ROOK, 4 + 56 * (COLOR as u8), 7 + 56 * (COLOR as u8));
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece(COLOR, piece, to, from);
                self.add_piece(enemy_color, PAWN, ((to as i8) + 8 * ((COLOR as i8) * 2 - 1)) as u8);
            }
            _ => {
                self.add_piece(COLOR, PAWN, from);
                self.remove_piece(COLOR, piece, to);

                if flags.contains(MoveFlags::CAPTURE) {
                    let captured_piece = self.captured_pieces_stack.pop().unwrap();
                    self.add_piece(enemy_color, captured_piece, to);
                }
            }
        }

        if COLOR == BLACK {
            self.fullmove_number -= 1;
        }

        self.active_color = COLOR;
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new()
    }
}
