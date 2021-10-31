use super::fen;
use super::movegen;
use super::movescan;
use super::movescan::Move;
use super::movescan::MoveFlags;
use super::zobrist;
use super::*;
use crate::cache::pawns::PawnHashTable;
use crate::engine::context::SearchStatistics;
use crate::evaluation::material;
use crate::evaluation::mobility;
use crate::evaluation::pawns;
use crate::evaluation::pst;
use crate::evaluation::safety;

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
    pub pawn_hash: u64,
    pub null_moves: u8,
    pub halfmove_clocks_stack: Vec<u16>,
    pub captured_pieces_stack: Vec<u8>,
    pub castling_rights_stack: Vec<CastlingRights>,
    pub en_passant_stack: Vec<u64>,
    pub hash_stack: Vec<u64>,
    pub pawn_hash_stack: Vec<u64>,
    pub material_scores: [i16; 2],
    pub pst_scores: [[i16; 2]; 2],
}

impl Bitboard {
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
            board.make_move(&parsed_move);
        }

        Ok(board)
    }

    pub fn get_moves(&self, moves: &mut [Move]) -> usize {
        let mut index = 0;
        index = movescan::scan_pawn_moves(self, moves, index);
        index = movescan::scan_piece_moves::<KNIGHT>(self, moves, index);
        index = movescan::scan_piece_moves::<BISHOP>(self, moves, index);
        index = movescan::scan_piece_moves::<ROOK>(self, moves, index);
        index = movescan::scan_piece_moves::<QUEEN>(self, moves, index);
        index = movescan::scan_piece_moves::<KING>(self, moves, index);

        index
    }

    pub fn make_move(&mut self, r#move: &Move) {
        let color = self.active_color;
        let enemy_color = self.active_color ^ 1;

        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(from);

        self.halfmove_clocks_stack.push(self.halfmove_clock);
        self.castling_rights_stack.push(self.castling_rights);
        self.en_passant_stack.push(self.en_passant);
        self.hash_stack.push(self.hash);
        self.pawn_hash_stack.push(self.pawn_hash);

        if self.en_passant != 0 {
            self.hash ^= zobrist::get_en_passant_hash((bit_scan(self.en_passant) % 8) as u8);
            self.en_passant = 0;
        }

        match flags {
            MoveFlags::QUIET => {
                self.move_piece(color, piece, from, to);
                self.hash ^= zobrist::get_piece_hash(color, piece, from);
                self.hash ^= zobrist::get_piece_hash(color, piece, to);

                if piece == PAWN {
                    self.pawn_hash ^= zobrist::get_piece_hash(color, piece, from);
                    self.pawn_hash ^= zobrist::get_piece_hash(color, piece, to);
                }
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(color, piece, from, to);
                self.hash ^= zobrist::get_piece_hash(color, piece, from);
                self.hash ^= zobrist::get_piece_hash(color, piece, to);

                self.pawn_hash ^= zobrist::get_piece_hash(color, piece, from);
                self.pawn_hash ^= zobrist::get_piece_hash(color, piece, to);

                self.en_passant = 1u64 << ((to as i8) + 8 * ((color as i8) * 2 - 1));
                self.hash ^= zobrist::get_en_passant_hash((bit_scan(self.en_passant) % 8) as u8);
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.get_piece(to);
                self.captured_pieces_stack.push(captured_piece);

                self.remove_piece(enemy_color, captured_piece, to);
                self.hash ^= zobrist::get_piece_hash(enemy_color, captured_piece, to);

                if captured_piece == PAWN {
                    self.pawn_hash ^= zobrist::get_piece_hash(enemy_color, captured_piece, to);
                }

                self.move_piece(color, piece, from, to);
                self.hash ^= zobrist::get_piece_hash(color, piece, from);
                self.hash ^= zobrist::get_piece_hash(color, piece, to);

                if piece == PAWN {
                    self.pawn_hash ^= zobrist::get_piece_hash(color, piece, from);
                    self.pawn_hash ^= zobrist::get_piece_hash(color, piece, to);
                }
            }
            MoveFlags::SHORT_CASTLING => {
                let king_from = 3 + 56 * (color as u8);
                let king_to = 1 + 56 * (color as u8);

                self.move_piece(color, KING, king_from, king_to);
                self.hash ^= zobrist::get_piece_hash(color, KING, king_from);
                self.hash ^= zobrist::get_piece_hash(color, KING, king_to);

                let rook_from = 0 + 56 * (color as u8);
                let rook_to = 2 + 56 * (color as u8);

                self.move_piece(color, ROOK, rook_from, rook_to);
                self.hash ^= zobrist::get_piece_hash(color, ROOK, rook_from);
                self.hash ^= zobrist::get_piece_hash(color, ROOK, rook_to);
            }
            MoveFlags::LONG_CASTLING => {
                let king_from = 3 + 56 * (color as u8);
                let king_to = 5 + 56 * (color as u8);

                self.move_piece(color, KING, 3 + 56 * (color as u8), 5 + 56 * (color as u8));
                self.hash ^= zobrist::get_piece_hash(color, KING, king_from);
                self.hash ^= zobrist::get_piece_hash(color, KING, king_to);

                let rook_from = 7 + 56 * (color as u8);
                let rook_to = 4 + 56 * (color as u8);

                self.move_piece(color, ROOK, 7 + 56 * (color as u8), 4 + 56 * (color as u8));
                self.hash ^= zobrist::get_piece_hash(color, ROOK, rook_from);
                self.hash ^= zobrist::get_piece_hash(color, ROOK, rook_to);
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece(color, piece, from, to);
                self.hash ^= zobrist::get_piece_hash(color, piece, from);
                self.hash ^= zobrist::get_piece_hash(color, piece, to);

                self.pawn_hash ^= zobrist::get_piece_hash(color, piece, from);
                self.pawn_hash ^= zobrist::get_piece_hash(color, piece, to);

                let enemy_pawn_field_index = ((to as i8) + 8 * ((color as i8) * 2 - 1)) as u8;

                self.remove_piece(enemy_color, PAWN, enemy_pawn_field_index);
                self.hash ^= zobrist::get_piece_hash(enemy_color, piece, enemy_pawn_field_index);
                self.pawn_hash ^= zobrist::get_piece_hash(enemy_color, piece, enemy_pawn_field_index);
            }
            _ => {
                let promotion_piece = r#move.get_promotion_piece();
                if flags.contains(MoveFlags::CAPTURE) {
                    let captured_piece = self.get_piece(to);
                    self.captured_pieces_stack.push(captured_piece);

                    self.remove_piece(enemy_color, captured_piece, to);
                    self.hash ^= zobrist::get_piece_hash(enemy_color, captured_piece, to);
                }

                self.remove_piece(color, PAWN, from);
                self.hash ^= zobrist::get_piece_hash(color, PAWN, from);
                self.pawn_hash ^= zobrist::get_piece_hash(color, PAWN, from);

                self.add_piece(color, promotion_piece, to);
                self.hash ^= zobrist::get_piece_hash(color, promotion_piece, to);
            }
        }

        if piece == KING {
            self.castling_rights &= match color {
                WHITE => {
                    self.hash ^= zobrist::get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                    self.hash ^= zobrist::get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_LONG_CASTLING);

                    !CastlingRights::WHITE_CASTLING
                }
                BLACK => {
                    self.hash ^= zobrist::get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                    self.hash ^= zobrist::get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_LONG_CASTLING);

                    !CastlingRights::BLACK_CASTLING
                }
                _ => panic!("Invalid value: color={}", color),
            };

            self.pawn_hash ^= zobrist::get_piece_hash(color, KING, from);
            self.pawn_hash ^= zobrist::get_piece_hash(color, KING, to);
        }

        if piece == ROOK {
            match color {
                WHITE => {
                    if from == 0 {
                        self.hash ^= zobrist::get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                        self.castling_rights &= !CastlingRights::WHITE_SHORT_CASTLING;
                    } else if from == 7 {
                        self.hash ^= zobrist::get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
                        self.castling_rights &= !CastlingRights::WHITE_LONG_CASTLING;
                    }
                }
                BLACK => {
                    if from == 56 {
                        self.hash ^= zobrist::get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                        self.castling_rights &= !CastlingRights::BLACK_SHORT_CASTLING;
                    } else if from == 63 {
                        self.hash ^= zobrist::get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
                        self.castling_rights &= !CastlingRights::BLACK_LONG_CASTLING;
                    }
                }
                _ => panic!("Invalid value: color={}", color),
            }
        }

        if color == BLACK {
            self.fullmove_number += 1;
        }

        if piece == PAWN || flags.contains(MoveFlags::CAPTURE) {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.active_color = enemy_color;
        self.hash ^= zobrist::get_active_color_hash();
    }

    pub fn undo_move(&mut self, r#move: &Move) {
        let color = self.active_color ^ 1;
        let enemy_color = self.active_color;

        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(to);

        self.halfmove_clock = self.halfmove_clocks_stack.pop().unwrap();
        self.castling_rights = self.castling_rights_stack.pop().unwrap();
        self.en_passant = self.en_passant_stack.pop().unwrap();
        self.hash = self.hash_stack.pop().unwrap();
        self.pawn_hash = self.pawn_hash_stack.pop().unwrap();

        match flags {
            MoveFlags::QUIET => {
                self.move_piece(color, piece, to, from);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(color, piece, to, from);
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.captured_pieces_stack.pop().unwrap();

                self.move_piece(color, piece, to, from);
                self.add_piece(enemy_color, captured_piece, to);
            }
            MoveFlags::SHORT_CASTLING => {
                self.move_piece(color, KING, 1 + 56 * (color as u8), 3 + 56 * (color as u8));
                self.move_piece(color, ROOK, 2 + 56 * (color as u8), 0 + 56 * (color as u8));
            }
            MoveFlags::LONG_CASTLING => {
                self.move_piece(color, KING, 5 + 56 * (color as u8), 3 + 56 * (color as u8));
                self.move_piece(color, ROOK, 4 + 56 * (color as u8), 7 + 56 * (color as u8));
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece(color, piece, to, from);
                self.add_piece(enemy_color, PAWN, ((to as i8) + 8 * ((color as i8) * 2 - 1)) as u8);
            }
            _ => {
                self.add_piece(color, PAWN, from);
                self.remove_piece(color, piece, to);

                if flags.contains(MoveFlags::CAPTURE) {
                    let captured_piece = self.captured_pieces_stack.pop().unwrap();
                    self.add_piece(enemy_color, captured_piece, to);
                }
            }
        }

        if color == BLACK {
            self.fullmove_number -= 1;
        }

        self.active_color = color;
    }

    pub fn make_null_move(&mut self) {
        let color = self.active_color;
        let enemy_color = self.active_color ^ 1;

        self.halfmove_clocks_stack.push(self.halfmove_clock);
        self.castling_rights_stack.push(self.castling_rights);
        self.en_passant_stack.push(self.en_passant);
        self.hash_stack.push(self.hash);

        if self.en_passant != 0 {
            self.hash ^= zobrist::get_en_passant_hash((bit_scan(self.en_passant) % 8) as u8);
            self.en_passant = 0;
        }

        if color == BLACK {
            self.fullmove_number += 1;
        }

        self.active_color = enemy_color;
        self.hash ^= zobrist::get_active_color_hash();
        self.null_moves += 1;
    }

    pub fn undo_null_move(&mut self) {
        let color = self.active_color ^ 1;

        self.halfmove_clock = self.halfmove_clocks_stack.pop().unwrap();
        self.castling_rights = self.castling_rights_stack.pop().unwrap();
        self.en_passant = self.en_passant_stack.pop().unwrap();
        self.hash = self.hash_stack.pop().unwrap();

        if color == BLACK {
            self.fullmove_number -= 1;
        }

        self.active_color = color;
        self.null_moves -= 1;
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

    pub fn get_attacking_pieces(&self, color: u8, field_index: u8) -> u8 {
        /*
            0 - pawn
            1 - knight/bishop
            2 - knight/bishop
            3 - knight/bishop
            4 - rook
            5 - rook
            6 - queen
            7 - king
        */

        let mut result = 0;
        let enemy_color = color ^ 1;
        let occupancy = self.occupancy[WHITE as usize] | self.occupancy[BLACK as usize];

        let bishops_rooks = self.pieces[enemy_color as usize][BISHOP as usize] | self.pieces[enemy_color as usize][ROOK as usize];
        let rooks_queens = self.pieces[enemy_color as usize][ROOK as usize] | self.pieces[enemy_color as usize][QUEEN as usize];
        let bishops_queens = self.pieces[enemy_color as usize][BISHOP as usize] | self.pieces[enemy_color as usize][QUEEN as usize];

        let king_attacks = movegen::get_king_moves(field_index as usize);
        if (king_attacks & self.pieces[enemy_color as usize][KING as usize]) != 0 {
            result |= 1 << 7;
        }

        let queen_attacks = movegen::get_queen_moves(occupancy & !bishops_rooks, field_index as usize);
        if (queen_attacks & self.pieces[enemy_color as usize][QUEEN as usize]) != 0 {
            result |= 1 << 6;
        }

        let rook_attacks = movegen::get_rook_moves(occupancy & !rooks_queens, field_index as usize);
        let attacking_rooks = rook_attacks & self.pieces[enemy_color as usize][ROOK as usize];
        if attacking_rooks != 0 {
            result |= match bit_count(attacking_rooks) {
                1 => 1 << 4,
                _ => 3 << 4,
            };
        }

        let mut attacking_knights_bishops_count = 0;

        let knight_attacks = movegen::get_knight_moves(field_index as usize);
        let enemy_knights = self.pieces[enemy_color as usize][KNIGHT as usize];
        let attacking_knights = knight_attacks & enemy_knights;
        if (knight_attacks & enemy_knights) != 0 {
            attacking_knights_bishops_count += bit_count(attacking_knights);
        }

        let bishop_attacks = movegen::get_bishop_moves(occupancy & !bishops_queens, field_index as usize);
        let enemy_bishops = self.pieces[enemy_color as usize][BISHOP as usize];
        let attacking_bishops = bishop_attacks & enemy_bishops;
        if (bishop_attacks & enemy_bishops) != 0 {
            attacking_knights_bishops_count += bit_count(attacking_bishops);
        }

        if attacking_knights_bishops_count != 0 {
            result |= match attacking_knights_bishops_count {
                1 => 1 << 1,
                2 => 3 << 1,
                _ => 7 << 1,
            };
        }

        let field = 1u64 << field_index;
        let potential_enemy_pawns = king_attacks & self.pieces[enemy_color as usize][PAWN as usize];
        let attacking_enemy_pawns = match color {
            WHITE => field & ((potential_enemy_pawns >> 7) | (potential_enemy_pawns >> 9)),
            BLACK => field & ((potential_enemy_pawns << 7) | (potential_enemy_pawns << 9)),
            _ => panic!("Invalid value: color={}", color),
        };

        if attacking_enemy_pawns != 0 {
            result |= 1;
        }

        result
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
        self.material_scores[color as usize] += material::get_value(piece);

        self.pst_scores[color as usize][OPENING as usize] += pst::get_value(piece, color, OPENING, field);
        self.pst_scores[color as usize][ENDING as usize] += pst::get_value(piece, color, ENDING, field);
    }

    pub fn remove_piece(&mut self, color: u8, piece: u8, field: u8) {
        self.pieces[color as usize][piece as usize] &= !(1u64 << field);
        self.occupancy[color as usize] &= !(1u64 << field);
        self.piece_table[field as usize] = u8::MAX;
        self.material_scores[color as usize] -= material::get_value(piece);

        self.pst_scores[color as usize][OPENING as usize] -= pst::get_value(piece, color, OPENING, field);
        self.pst_scores[color as usize][ENDING as usize] -= pst::get_value(piece, color, ENDING, field);
    }

    pub fn move_piece(&mut self, color: u8, piece: u8, from: u8, to: u8) {
        self.pieces[color as usize][piece as usize] ^= (1u64 << from) | (1u64 << to);
        self.occupancy[color as usize] ^= (1u64 << from) | (1u64 << to);

        self.piece_table[to as usize] = self.piece_table[from as usize];
        self.piece_table[from as usize] = u8::MAX;

        self.pst_scores[color as usize][OPENING as usize] -= pst::get_value(piece, color, OPENING, from);
        self.pst_scores[color as usize][ENDING as usize] -= pst::get_value(piece, color, ENDING, from);
        self.pst_scores[color as usize][OPENING as usize] += pst::get_value(piece, color, OPENING, to);
        self.pst_scores[color as usize][ENDING as usize] += pst::get_value(piece, color, ENDING, to);
    }

    pub fn to_fen(&self) -> String {
        fen::board_to_fen(self)
    }

    pub fn recalculate_hash(&mut self) {
        zobrist::recalculate_hash(self);
    }

    pub fn recalculate_pawn_hash(&mut self) {
        zobrist::recalculate_pawn_hash(self);
    }

    pub fn evaluate(&self, pawn_hash_table: &mut PawnHashTable, statistics: &mut SearchStatistics) -> i16 {
        let mut white_attack_mask = 0;
        let mut black_attack_mask = 0;
        let mobility_score = mobility::evaluate(self, &mut white_attack_mask, &mut black_attack_mask);

        material::evaluate(self)
            + pst::evaluate(self)
            + pawns::evaluate(self, pawn_hash_table, statistics)
            + safety::evaluate(self, white_attack_mask, black_attack_mask)
            + mobility_score
    }

    pub fn evaluate_without_cache(&self) -> i16 {
        let mut white_attack_mask = 0;
        let mut black_attack_mask = 0;
        let mobility_score = mobility::evaluate(self, &mut white_attack_mask, &mut black_attack_mask);

        material::evaluate(self)
            + pst::evaluate(self)
            + pawns::evaluate_without_cache(self)
            + safety::evaluate(self, white_attack_mask, black_attack_mask)
            + mobility_score
    }

    pub fn recalculate_incremental_values(&mut self) {
        material::recalculate_incremental_values(self);
        pst::recalculate_incremental_values(self);
    }

    pub fn is_threefold_repetition_draw(&self) -> bool {
        if self.null_moves > 0 {
            return false;
        }

        let mut repetitions_count = 1;
        for hash in &self.hash_stack {
            if *hash == self.hash {
                repetitions_count += 1;
            }
        }

        repetitions_count >= 3
    }

    pub fn is_fifty_move_rule_draw(&self) -> bool {
        if self.null_moves > 0 {
            return false;
        }

        self.halfmove_clock >= 100
    }

    pub fn get_game_phase(&self) -> f32 {
        let initial_material = 7920;
        let total_material = self.material_scores[WHITE as usize] + self.material_scores[BLACK as usize] - 20000;

        (total_material as f32) / (initial_material as f32)
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard {
            pieces: [[0; 6], [0; 6]],
            occupancy: [0; 2],
            piece_table: [u8::MAX; 64],
            castling_rights: CastlingRights::NONE,
            en_passant: 0,
            halfmove_clock: 0,
            fullmove_number: 0,
            active_color: WHITE,
            hash: 0,
            pawn_hash: 0,
            null_moves: 0,
            halfmove_clocks_stack: Vec::with_capacity(32),
            captured_pieces_stack: Vec::with_capacity(32),
            castling_rights_stack: Vec::with_capacity(32),
            en_passant_stack: Vec::with_capacity(32),
            hash_stack: Vec::with_capacity(32),
            pawn_hash_stack: Vec::with_capacity(32),
            material_scores: [0; 2],
            pst_scores: [[0, 2]; 2],
        }
    }
}
