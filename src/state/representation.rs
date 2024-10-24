use super::movescan;
use super::movescan::Move;
use super::movescan::MoveFlags;
use super::text::fen;
use super::*;
use crate::cache::pawns::PHTable;
use crate::engine;
use crate::engine::stats::SearchStats;
use crate::evaluation::material;
use crate::evaluation::mobility;
use crate::evaluation::mobility::EvalAux;
use crate::evaluation::pawns;
use crate::evaluation::pst;
use crate::evaluation::pst::*;
use crate::evaluation::safety;
use crate::evaluation::*;
use crate::tablebases;
use crate::utils::assert_fast;
use crate::utils::bitflags::BitFlags;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::panic_fast;
use crate::Moves;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::MaybeUninit;

#[allow(non_snake_case)]
pub mod CastlingRights {
    pub const NONE: u8 = 0;
    pub const WHITE_SHORT_CASTLING: u8 = 1;
    pub const WHITE_LONG_CASTLING: u8 = 2;
    pub const BLACK_SHORT_CASTLING: u8 = 4;
    pub const BLACK_LONG_CASTLING: u8 = 8;

    pub const WHITE_CASTLING: u8 = WHITE_SHORT_CASTLING | WHITE_LONG_CASTLING;
    pub const BLACK_CASTLING: u8 = BLACK_SHORT_CASTLING | BLACK_LONG_CASTLING;
    pub const ALL: u8 = WHITE_CASTLING | BLACK_CASTLING;
}

#[derive(Clone)]
pub struct Board {
    pub pieces: [[u64; 6]; 2],
    pub occupancy: [u64; 2],
    pub piece_table: [u8; 64],
    pub fullmove_number: u16,
    pub stm: usize,
    pub null_moves: u8,
    pub game_phase: u8,
    pub state: BoardState,
    pub state_stack: Vec<BoardState>,
    pub pawn_attacks: [u64; 2],
}

#[derive(Copy, Clone)]
pub struct BoardState {
    pub halfmove_clock: u16,
    pub castling_rights: u8,
    pub en_passant: u64,
    pub hash: u64,
    pub pawn_hash: u64,
    pub captured_piece: u8,
    pub pst_score: PackedEval,
}

impl Board {
    /// Constructs a new instance of [Board] with initial position.
    pub fn new_initial_position() -> Self {
        Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    /// Constructs a new instance of [Board]. Returns [Err] with proper error message if `fen` couldn't be parsed correctly.
    pub fn new_from_fen(fen: &str) -> Result<Self, String> {
        fen::fen_to_board(fen)
    }

    /// Constructs a new instance of [Board] with position specified by list of `moves`. Returns [Err] with proper error message is `moves` couldn't be parsed correctly.
    pub fn new_from_moves(moves: &[&str]) -> Result<Self, String> {
        let mut board = Board::new_initial_position();
        for premade_move in moves {
            let parsed_move = Move::from_long_notation(premade_move, &board)?;
            board.make_move(parsed_move);
        }

        Ok(board)
    }

    /// Generates all possible non-captures (if `CAPTURES` is false) or all possible captures (if `CAPTURES` is true) at the current position, stores
    /// them into `moves` list (starting from `index`) and returns index of the first free slot. Use `evasion_mask` with value different
    /// than `u64::MAX` to restrict generator to the specified squares (useful during checks).
    pub fn get_moves<const CAPTURES: bool>(&self, moves: &mut Moves, mut index: usize, evasion_mask: u64) -> usize {
        index = movescan::scan_pawn_moves::<CAPTURES>(self, moves, index, evasion_mask);
        index = movescan::scan_piece_moves::<KNIGHT, CAPTURES>(self, moves, index, evasion_mask);
        index = movescan::scan_piece_moves::<BISHOP, CAPTURES>(self, moves, index, evasion_mask);
        index = movescan::scan_piece_moves::<ROOK, CAPTURES>(self, moves, index, evasion_mask);
        index = movescan::scan_piece_moves::<QUEEN, CAPTURES>(self, moves, index, evasion_mask);
        index = movescan::scan_piece_moves::<KING, CAPTURES>(self, moves, index, evasion_mask);

        index
    }

    /// Generates all possible moves (non-captures and captures) at the current position, stores them into `moves` list (starting from `index`) and returns
    /// index of the first free slot. Use `evasion_mask` with value different than `u64::MAX` to restrict generator to the specified squares (useful during checks).
    pub fn get_all_moves(&self, moves: &mut Moves, evasion_mask: u64) -> usize {
        let mut index = 0;
        index = self.get_moves::<true>(moves, index, evasion_mask);
        index = self.get_moves::<false>(moves, index, evasion_mask);

        index
    }

    /// Makes `r#move`, with the assumption that it's perfectly valid at the current position (otherwise, internal state can be irreversibly corrupted).
    ///
    /// Steps of making a move:
    ///  - preserve halfmove clock, castling rights, en passant bitboard, board hash and pawn hash
    ///  - update piece bitboards
    ///  - update board hash and pawn hash
    ///  - update en passant bitboard if needed
    ///  - update castling rights if needed
    ///  - increase fullmove number if needed
    ///  - increase halfmove clock if needed
    ///  - switch active color
    pub fn make_move(&mut self, r#move: Move) {
        assert_fast!(!r#move.is_empty());
        self.push_state();

        let stm = self.stm;
        let nstm = self.stm ^ 1;
        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(from);

        if self.state.en_passant != 0 {
            self.state.hash ^= zobrist::get_en_passant_hash(self.state.en_passant.bit_scan() & 7);
            self.state.en_passant = 0;
        }

        match flags {
            MoveFlags::SINGLE_PUSH => {
                self.move_piece::<false>(stm, piece, from, to);
                self.state.hash ^= zobrist::get_piece_hash(stm, piece, from);
                self.state.hash ^= zobrist::get_piece_hash(stm, piece, to);

                if piece == PAWN {
                    self.state.pawn_hash ^= zobrist::get_piece_hash(stm, piece, from);
                    self.state.pawn_hash ^= zobrist::get_piece_hash(stm, piece, to);
                    self.recalculate_pawn_attacks(stm);
                }
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece::<false>(stm, piece, from, to);
                self.state.hash ^= zobrist::get_piece_hash(stm, piece, from);
                self.state.hash ^= zobrist::get_piece_hash(stm, piece, to);

                self.state.pawn_hash ^= zobrist::get_piece_hash(stm, piece, from);
                self.state.pawn_hash ^= zobrist::get_piece_hash(stm, piece, to);

                let sign = (stm as i8) * 2 - 1;
                self.state.en_passant = 1u64 << ((to as i8) + sign * 8);
                self.state.hash ^= zobrist::get_en_passant_hash(self.state.en_passant.bit_scan() & 7);

                self.recalculate_pawn_attacks(stm);
            }
            MoveFlags::CAPTURE => {
                let captured_piece = self.get_piece(to);
                self.remove_piece::<false>(nstm, captured_piece, to);
                self.state.hash ^= zobrist::get_piece_hash(nstm, captured_piece, to);

                if captured_piece == PAWN {
                    self.state.pawn_hash ^= zobrist::get_piece_hash(nstm, captured_piece, to);
                    self.recalculate_pawn_attacks(nstm);
                }

                self.move_piece::<false>(stm, piece, from, to);
                self.state.hash ^= zobrist::get_piece_hash(stm, piece, from);
                self.state.hash ^= zobrist::get_piece_hash(stm, piece, to);

                if piece == PAWN {
                    self.state.pawn_hash ^= zobrist::get_piece_hash(stm, piece, from);
                    self.state.pawn_hash ^= zobrist::get_piece_hash(stm, piece, to);
                    self.recalculate_pawn_attacks(stm);
                }

                self.state.captured_piece = captured_piece as u8;
            }
            MoveFlags::SHORT_CASTLING => {
                let king_from = 3 + 56 * stm;
                let king_to = 1 + 56 * stm;

                self.move_piece::<false>(stm, KING, king_from, king_to);
                self.state.hash ^= zobrist::get_piece_hash(stm, KING, king_from);
                self.state.hash ^= zobrist::get_piece_hash(stm, KING, king_to);

                let rook_from = 0 + 56 * stm;
                let rook_to = 2 + 56 * stm;

                self.move_piece::<false>(stm, ROOK, rook_from, rook_to);
                self.state.hash ^= zobrist::get_piece_hash(stm, ROOK, rook_from);
                self.state.hash ^= zobrist::get_piece_hash(stm, ROOK, rook_to);
            }
            MoveFlags::LONG_CASTLING => {
                let king_from = 3 + 56 * stm;
                let king_to = 5 + 56 * stm;

                self.move_piece::<false>(stm, KING, king_from, king_to);
                self.state.hash ^= zobrist::get_piece_hash(stm, KING, king_from);
                self.state.hash ^= zobrist::get_piece_hash(stm, KING, king_to);

                let rook_from = 7 + 56 * stm;
                let rook_to = 4 + 56 * stm;

                self.move_piece::<false>(stm, ROOK, rook_from, rook_to);
                self.state.hash ^= zobrist::get_piece_hash(stm, ROOK, rook_from);
                self.state.hash ^= zobrist::get_piece_hash(stm, ROOK, rook_to);
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece::<false>(stm, piece, from, to);
                self.state.hash ^= zobrist::get_piece_hash(stm, piece, from);
                self.state.hash ^= zobrist::get_piece_hash(stm, piece, to);

                self.state.pawn_hash ^= zobrist::get_piece_hash(stm, piece, from);
                self.state.pawn_hash ^= zobrist::get_piece_hash(stm, piece, to);

                let sign = (stm as isize) * 2 - 1;
                let enemy_pawn_square = ((to as isize) + sign * 8) as usize;

                self.remove_piece::<false>(nstm, PAWN, enemy_pawn_square);
                self.state.hash ^= zobrist::get_piece_hash(nstm, piece, enemy_pawn_square);
                self.state.pawn_hash ^= zobrist::get_piece_hash(nstm, piece, enemy_pawn_square);

                self.recalculate_pawn_attacks(stm);
                self.recalculate_pawn_attacks(nstm);
            }
            _ => {
                let promotion_piece = r#move.get_promotion_piece();
                if flags.contains(MoveFlags::CAPTURE) {
                    let captured_piece = self.get_piece(to);
                    self.remove_piece::<false>(nstm, captured_piece, to);
                    self.state.hash ^= zobrist::get_piece_hash(nstm, captured_piece, to);
                    self.state.captured_piece = captured_piece as u8;
                }

                self.remove_piece::<false>(stm, PAWN, from);
                self.state.hash ^= zobrist::get_piece_hash(stm, PAWN, from);
                self.state.pawn_hash ^= zobrist::get_piece_hash(stm, PAWN, from);

                self.add_piece::<false>(stm, promotion_piece, to);
                self.state.hash ^= zobrist::get_piece_hash(stm, promotion_piece, to);

                self.recalculate_pawn_attacks(stm);
            }
        }

        if piece == KING {
            self.state.castling_rights &= match stm {
                WHITE => {
                    self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                    self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::WHITE_LONG_CASTLING);

                    !CastlingRights::WHITE_CASTLING
                }
                BLACK => {
                    self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                    self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::BLACK_LONG_CASTLING);

                    !CastlingRights::BLACK_CASTLING
                }
                _ => panic_fast!("Invalid parameter: fen={}, stm={}", self, stm),
            };

            self.state.pawn_hash ^= zobrist::get_piece_hash(stm, KING, from);
            self.state.pawn_hash ^= zobrist::get_piece_hash(stm, KING, to);

            let from = if stm == WHITE { from } else { (1u64 << from).swap_bytes().bit_scan() };
            let to = if stm == WHITE { to } else { (1u64 << to).swap_bytes().bit_scan() };

            if KING_BUCKETS[from] != KING_BUCKETS[to] {
                pst::recalculate_incremental_values(self);
            }
        } else if piece == ROOK {
            match stm {
                WHITE => match from {
                    A1 => {
                        self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
                        self.state.castling_rights &= !CastlingRights::WHITE_LONG_CASTLING;
                    }
                    H1 => {
                        self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                        self.state.castling_rights &= !CastlingRights::WHITE_SHORT_CASTLING;
                    }
                    _ => {}
                },
                BLACK => match from {
                    A8 => {
                        self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
                        self.state.castling_rights &= !CastlingRights::BLACK_LONG_CASTLING;
                    }
                    H8 => {
                        self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                        self.state.castling_rights &= !CastlingRights::BLACK_SHORT_CASTLING;
                    }
                    _ => {}
                },
                _ => panic_fast!("Invalid parameter: fen={}, stm={}", self, stm),
            }
        }

        if self.state.captured_piece == ROOK as u8 {
            match nstm {
                WHITE => match to {
                    A1 => {
                        self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
                        self.state.castling_rights &= !CastlingRights::WHITE_LONG_CASTLING;
                    }
                    H1 => {
                        self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                        self.state.castling_rights &= !CastlingRights::WHITE_SHORT_CASTLING;
                    }
                    _ => {}
                },
                BLACK => match to {
                    A8 => {
                        self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
                        self.state.castling_rights &= !CastlingRights::BLACK_LONG_CASTLING;
                    }
                    H8 => {
                        self.state.hash ^= zobrist::get_castling_right_hash(self.state.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                        self.state.castling_rights &= !CastlingRights::BLACK_SHORT_CASTLING;
                    }
                    _ => {}
                },
                _ => panic_fast!("Invalid parameter: fen={}, stm={}", self, stm),
            }
        }

        if stm == BLACK {
            self.fullmove_number += 1;
        }

        if piece == PAWN || flags.contains(MoveFlags::CAPTURE) {
            self.state.halfmove_clock = 0;
        } else {
            self.state.halfmove_clock += 1;
        }

        self.state.hash ^= zobrist::get_stm_hash();
        self.stm = nstm;
    }

    /// Undoes `r#move`, with the assumption that it's perfectly valid at the current position (otherwise, internal state can be irreversibly corrupted).
    ///
    /// Steps of undoing a move:
    ///  - update piece bitboards
    ///  - decrease fullmove number if needed
    ///  - restore halfmove clock if needed
    ///  - switch active color
    ///  - restore halfmove clock, castling rights, en passant bitboard, board hash and pawn hash
    pub fn undo_move(&mut self, r#move: Move) {
        assert_fast!(!r#move.is_empty());

        let stm = self.stm ^ 1;
        let nstm = self.stm;
        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(to);
        let captured_piece = self.state.captured_piece as usize;

        match flags {
            MoveFlags::SINGLE_PUSH => {
                self.move_piece::<true>(stm, piece, to, from);

                if piece == PAWN {
                    self.recalculate_pawn_attacks(stm);
                }
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece::<true>(stm, piece, to, from);
                self.recalculate_pawn_attacks(stm);
            }
            MoveFlags::CAPTURE => {
                self.move_piece::<true>(stm, piece, to, from);
                self.add_piece::<true>(nstm, captured_piece, to);

                if piece == PAWN {
                    self.recalculate_pawn_attacks(stm);
                }

                if captured_piece == PAWN {
                    self.recalculate_pawn_attacks(nstm);
                }
            }
            MoveFlags::SHORT_CASTLING => {
                self.move_piece::<true>(stm, KING, 1 + 56 * stm, 3 + 56 * stm);
                self.move_piece::<true>(stm, ROOK, 2 + 56 * stm, 0 + 56 * stm);
            }
            MoveFlags::LONG_CASTLING => {
                self.move_piece::<true>(stm, KING, 5 + 56 * stm, 3 + 56 * stm);
                self.move_piece::<true>(stm, ROOK, 4 + 56 * stm, 7 + 56 * stm);
            }
            MoveFlags::EN_PASSANT => {
                let sign = (stm as isize) * 2 - 1;
                let enemy_pawn_square = ((to as isize) + sign * 8) as usize;

                self.move_piece::<true>(stm, piece, to, from);
                self.add_piece::<true>(nstm, PAWN, enemy_pawn_square);
                self.recalculate_pawn_attacks(stm);
                self.recalculate_pawn_attacks(nstm);
            }
            _ => {
                self.add_piece::<true>(stm, PAWN, from);
                self.remove_piece::<true>(stm, piece, to);
                self.recalculate_pawn_attacks(stm);

                if flags.contains(MoveFlags::CAPTURE) {
                    self.add_piece::<true>(nstm, captured_piece, to);
                }
            }
        }

        if stm == BLACK {
            self.fullmove_number -= 1;
        }

        self.stm = stm;
        self.pop_state();
    }

    /// Makes a null move, which is basically a switch of the active color with preservation of the internal state.
    ///
    /// Steps of making a null move:
    ///  - preserve halfmove clock, castling rights, en passant bitboard, board hash and pawn hash
    ///  - update en passant bitboard if needed
    ///  - increase fullmove number if needed
    ///  - increase null moves count
    ///  - switch active color
    pub fn make_null_move(&mut self) {
        self.push_state();

        if self.state.en_passant != 0 {
            self.state.hash ^= zobrist::get_en_passant_hash(self.state.en_passant.bit_scan() & 7);
            self.state.en_passant = 0;
        }

        if self.stm == BLACK {
            self.fullmove_number += 1;
        }

        self.null_moves += 1;
        self.stm ^= 1;
        self.state.hash ^= zobrist::get_stm_hash();
    }

    /// Undoes a null move, which is basically a switch of the active color with restoring of the internal state.
    ///
    /// Steps of undoing a null move:
    ///  - decrease fullmove number if needed
    ///  - switch active color
    ///  - decrease null moves count
    ///  - restore halfmove clock, castling rights, en passant bitboard, board hash and pawn hash
    pub fn undo_null_move(&mut self) {
        if self.stm == WHITE {
            self.fullmove_number -= 1;
        }

        self.stm ^= 1;
        self.null_moves -= 1;
        self.pop_state();
    }

    /// Preserves halfmove clock, castling rights, en passant bitboard, board hash, pawn hash and captured piece on the stack
    pub fn push_state(&mut self) {
        self.state_stack.push(self.state);
    }

    /// Restores halfmove clock, castling rights, en passant bitboard, board hash, pawn hash and captured piece from the stack
    pub fn pop_state(&mut self) {
        unsafe {
            self.state = self.state_stack.pop().unwrap_unchecked();
        }
    }

    /// Checks if the square specified by `square` is attacked by enemy, from the `color` perspective.
    pub fn is_square_attacked(&self, color: usize, square: usize) -> bool {
        assert_fast!(color < 2);
        assert_fast!(square < 64);

        let nstm = color ^ 1;
        let occupancy_bb = self.occupancy[WHITE] | self.occupancy[BLACK];

        let rook_queen_attacks_bb = movegen::get_rook_moves(occupancy_bb, square);
        let enemy_rooks_queens_bb = self.pieces[nstm][ROOK] | self.pieces[nstm][QUEEN];
        if (rook_queen_attacks_bb & enemy_rooks_queens_bb) != 0 {
            return true;
        }

        let bishop_queen_attacks_bb = movegen::get_bishop_moves(occupancy_bb, square);
        let enemy_bishops_queens_bb = self.pieces[nstm][BISHOP] | self.pieces[nstm][QUEEN];
        if (bishop_queen_attacks_bb & enemy_bishops_queens_bb) != 0 {
            return true;
        }

        let knight_attacks_bb = movegen::get_knight_moves(square);
        let enemy_knights_bb = self.pieces[nstm][KNIGHT];
        if (knight_attacks_bb & enemy_knights_bb) != 0 {
            return true;
        }

        let king_attacks_bb = movegen::get_king_moves(square);
        let enemy_kings_bb = self.pieces[nstm][KING];
        if (king_attacks_bb & enemy_kings_bb) != 0 {
            return true;
        }

        let square_bb = 1u64 << square;
        if (self.pawn_attacks[nstm] & square_bb) != 0 {
            return true;
        }

        false
    }

    /// Checks if any of the square specified by `squares` list is attacked by enemy, from the `color` perspective.
    pub fn are_squares_attacked(&self, color: usize, squares: &[usize]) -> bool {
        assert_fast!(color < 2);

        for square in squares {
            if self.is_square_attacked(color, *square) {
                return true;
            }
        }

        false
    }

    /// Gets a list of enemy pieces attacking a square specified by `squares_index`, from the `color` perspective. The encoding looks as follows:
    ///  - bit 0 - Pawn
    ///  - bit 1, 2, 3 - Knight/Bishop
    ///  - bit 4, 5 - Rook
    ///  - bit 6 - Queen
    ///  - bit 7 - King
    pub fn get_attacking_pieces(&self, color: usize, square: usize) -> usize {
        assert_fast!(color < 2);
        assert_fast!(square < 64);

        let mut result = 0;
        let nstm = color ^ 1;
        let occupancy_bb = self.occupancy[WHITE] | self.occupancy[BLACK];

        let rooks_queens_bb = self.pieces[nstm][ROOK] | self.pieces[nstm][QUEEN];
        let bishops_queens_bb = self.pieces[nstm][BISHOP] | self.pieces[nstm][QUEEN];

        let king_attacks_bb = movegen::get_king_moves(square);
        let attacking_kings_count = ((king_attacks_bb & self.pieces[nstm][KING]) != 0) as usize;
        result |= attacking_kings_count << 7;

        let rook_attacks_bb = movegen::get_rook_moves(occupancy_bb & !rooks_queens_bb, square);
        let attacking_rooks_count = (rook_attacks_bb & self.pieces[nstm][ROOK]).bit_count();
        let attacking_queens_count = ((rook_attacks_bb & self.pieces[nstm][QUEEN]) != 0) as usize;

        result |= match attacking_rooks_count {
            0 => 0,
            1 => 1 << 4,
            _ => 3 << 4,
        };
        result |= attacking_queens_count << 6;

        let knight_attacks_bb = movegen::get_knight_moves(square);
        let attacking_knights_count = (knight_attacks_bb & self.pieces[nstm][KNIGHT]).bit_count();

        let bishop_attacks_bb = movegen::get_bishop_moves(occupancy_bb & !bishops_queens_bb, square);
        let attacking_bishops_count = (bishop_attacks_bb & self.pieces[nstm][BISHOP]).bit_count();
        let attacking_knights_bishops_count = attacking_knights_count + attacking_bishops_count;
        let attacking_queens_count = ((bishop_attacks_bb & self.pieces[nstm][QUEEN]) != 0) as usize;

        result |= match attacking_knights_bishops_count {
            0 => 0,
            1 => 1 << 1,
            2 => 3 << 1,
            _ => 7 << 1,
        };
        result |= attacking_queens_count << 6;

        let square_bb = 1u64 << square;
        let attacking_pawns_count = ((self.pawn_attacks[nstm] & square_bb) != 0) as usize;

        result |= attacking_pawns_count;
        result
    }

    /// Check if the king of the `color` side is checked.
    pub fn is_king_checked(&self, color: usize) -> bool {
        assert_fast!(color < 2);

        if self.pieces[color][KING] == 0 {
            return false;
        }

        self.is_square_attacked(color, (self.pieces[color][KING]).bit_scan())
    }

    /// Gets piece on the square specified by `square`.
    pub fn get_piece(&self, square: usize) -> usize {
        assert_fast!(square < 64);

        let piece = self.piece_table[square];
        if piece == u8::MAX {
            return usize::MAX;
        }

        piece as usize
    }

    /// Gets piece's color on the square specified by `square`. Returns `u8::MAX` if there is no piece there.
    pub fn get_piece_color(&self, square: usize) -> usize {
        assert_fast!(square < 64);

        let piece = self.piece_table[square];
        if piece == u8::MAX {
            return usize::MAX;
        }

        (((1u64 << square) & self.occupancy[WHITE]) == 0) as usize
    }

    /// Adds `piece` on the `square` with the specified `color`, also updates occupancy and incremental values.
    pub fn add_piece<const UNDO: bool>(&mut self, color: usize, piece: usize, mut square: usize) {
        assert_fast!(color < 2);
        assert_fast!(piece < 6);
        assert_fast!(square < 64);

        self.pieces[color][piece] |= 1u64 << square;
        self.occupancy[color] |= 1u64 << square;
        self.piece_table[square] = piece as u8;
        self.game_phase += PIECE_PHASE_VALUES[piece];

        if !UNDO {
            let mut king_square = self.pieces[color][KING].bit_scan() % 64;

            if color == BLACK {
                king_square = (1u64 << king_square).swap_bytes().bit_scan();
                square = (1u64 << square).swap_bytes().bit_scan();
            }

            let sign = -(color as i16 * 2 - 1);
            self.state.pst_score += sign * pst::get_pst_value(piece, king_square, square);
        }
    }

    /// Removes `piece` on the `square` with the specified `color`, also updates occupancy and incremental values.
    pub fn remove_piece<const UNDO: bool>(&mut self, color: usize, piece: usize, mut square: usize) {
        assert_fast!(color < 2);
        assert_fast!(piece < 6);
        assert_fast!(square < 64);

        self.pieces[color][piece] &= !(1u64 << square);
        self.occupancy[color] &= !(1u64 << square);
        self.piece_table[square] = u8::MAX;
        self.game_phase -= PIECE_PHASE_VALUES[piece];

        if !UNDO {
            let mut king_square = self.pieces[color][KING].bit_scan() % 64;

            if color == BLACK {
                king_square = (1u64 << king_square).swap_bytes().bit_scan();
                square = (1u64 << square).swap_bytes().bit_scan();
            }

            let sign = -(color as i16 * 2 - 1);
            self.state.pst_score -= sign * pst::get_pst_value(piece, king_square, square);
        }
    }

    /// Moves `piece` from the square specified by `from` to the square specified by `to` with the specified `color`, also updates occupancy and incremental values.
    pub fn move_piece<const UNDO: bool>(&mut self, color: usize, piece: usize, mut from: usize, mut to: usize) {
        assert_fast!(color < 2);
        assert_fast!(piece < 6);
        assert_fast!(from < 64);
        assert_fast!(to < 64);

        self.pieces[color][piece] ^= (1u64 << from) | (1u64 << to);
        self.occupancy[color] ^= (1u64 << from) | (1u64 << to);

        self.piece_table[to] = self.piece_table[from];
        self.piece_table[from] = u8::MAX;

        if !UNDO {
            let mut king_square = self.pieces[color][KING].bit_scan() % 64;

            if color == BLACK {
                king_square = (1u64 << king_square).swap_bytes().bit_scan();
                from = (1u64 << from).swap_bytes().bit_scan();
                to = (1u64 << to).swap_bytes().bit_scan();
            }

            let sign = -(color as i16 * 2 - 1);
            self.state.pst_score -= sign * pst::get_pst_value(piece, king_square, from);
            self.state.pst_score += sign * pst::get_pst_value(piece, king_square, to);
        }
    }

    /// Recalculates board's hashes entirely.
    pub fn recalculate_hashes(&mut self) {
        zobrist::recalculate_hash(self);
        zobrist::recalculate_pawn_hash(self);
    }

    /// Recalculate pawn attacks for the specific `color`.
    pub fn recalculate_pawn_attacks(&mut self, color: usize) {
        assert_fast!(color < 2);

        let pawns_bb = self.pieces[color][PAWN];
        self.pawn_attacks[color] = match color {
            WHITE => ((pawns_bb & !FILE_A_BB) << 9) | ((pawns_bb & !FILE_H_BB) << 7),
            BLACK => ((pawns_bb & !FILE_A_BB) >> 7) | ((pawns_bb & !FILE_H_BB) >> 9),
            _ => panic_fast!("Invalid value: color={}", color),
        };
    }

    /// Runs full evaluation (material, piece-square tables, mobility, pawns structure and safety) of the current position, using `phtable` to store pawn
    /// evaluations and `stats` to gather diagnostic data. Returns score from the `color` perspective (more than 0 when advantage, less than 0 when disadvantage).
    pub fn evaluate(&self, color: usize, phtable: &PHTable, stats: &mut SearchStats) -> i16 {
        assert_fast!(color < 2);

        let mut white_aux = EvalAux::default();
        let mut black_aux = EvalAux::default();

        let material_eval = material::evaluate(self);
        let pst_eval = pst::evaluate(self);
        let mobility_eval = mobility::evaluate(self, &mut white_aux, &mut black_aux);
        let safety_eval = safety::evaluate(self, &white_aux, &black_aux);
        let pawns_eval = pawns::evaluate(self, phtable, stats);

        let eval = material_eval + pst_eval + mobility_eval + safety_eval + pawns_eval;
        let sign = -((color as i16) * 2 - 1);

        sign * eval.taper_score(self.game_phase)
    }

    /// Runs full evaluation (material, piece-square tables, mobility, pawns structure and safety) of the current position.
    /// Returns score from the `color` perspective (more than 0 when advantage, less than 0 when disadvantage).
    pub fn evaluate_without_cache(&self, color: usize) -> i16 {
        assert_fast!(color < 2);

        let mut white_aux = EvalAux::default();
        let mut black_aux = EvalAux::default();

        let material_eval = material::evaluate(self);
        let pst_eval = pst::evaluate(self);
        let mobility_eval = mobility::evaluate(self, &mut white_aux, &mut black_aux);
        let safety_eval = safety::evaluate(self, &white_aux, &black_aux);
        let pawns_eval = pawns::evaluate_without_cache(self);

        let eval = material_eval + pst_eval + mobility_eval + safety_eval + pawns_eval;
        let sign = -((color as i16) * 2 - 1);

        sign * eval.taper_score(self.game_phase)
    }

    /// Runs fast evaluations, considering only material and piece-square tables. Returns score from the `color` perspective (more than 0 when
    /// advantage, less than 0 when disadvantage).
    pub fn evaluate_fast(&self, color: usize, phtable: &PHTable, stats: &mut SearchStats) -> i16 {
        assert_fast!(color < 2);

        let material_eval = material::evaluate(self);
        let pst_eval = pst::evaluate(self);
        let pawns_eval = pawns::evaluate(self, phtable, stats);

        let eval = material_eval + pst_eval + pawns_eval;
        let sign = -((color as i16) * 2 - 1);

        sign * eval.taper_score(self.game_phase)
    }

    /// Recalculates incremental values (material and piece-square tables) entirely.
    pub fn recalculate_incremental_values(&mut self) {
        pst::recalculate_incremental_values(self);
    }

    /// Checks if there's repetition draw with the specified `threshold` (should be 3 in the most cases) at the current position.
    pub fn is_repetition_draw(&self, threshold: i32) -> bool {
        if self.state_stack.len() < 6 || self.null_moves > 0 {
            return false;
        }

        let mut repetitions_count = 1;
        let from = self.state_stack.len().saturating_sub(self.state.halfmove_clock as usize);
        let to = self.state_stack.len() - 1;

        for hash_index in (from..to).rev().step_by(2) {
            assert_fast!(hash_index < self.state_stack.len());

            if self.state_stack[hash_index].hash == self.state.hash {
                repetitions_count += 1;

                if repetitions_count >= threshold {
                    return true;
                }
            }
        }

        false
    }

    /// Checks if there's fifty move rule draw at the current position.
    pub fn is_fifty_move_rule_draw(&self) -> bool {
        if self.null_moves > 0 {
            return false;
        }

        self.state.halfmove_clock >= 100
    }

    /// Checks if there's an inssuficient material draw:
    ///  - King vs King
    ///  - King + Knight/Bishop vs King
    ///  - King + Bishop (same color) vs King + Bishop (same color)
    pub fn is_insufficient_material_draw(&self) -> bool {
        let pawns_bb = self.pieces[WHITE][PAWN] | self.pieces[BLACK][PAWN];
        let pawns_count = pawns_bb.bit_count();

        if self.game_phase <= 2 && pawns_count == 0 {
            // King vs King
            if (self.occupancy[WHITE] | self.occupancy[BLACK]).bit_count() == 2 {
                return true;
            }

            let light_pieces_bb = self.pieces[WHITE][KNIGHT] | self.pieces[WHITE][BISHOP] | self.pieces[BLACK][KNIGHT] | self.pieces[BLACK][BISHOP];
            let light_pieces_count = light_pieces_bb.bit_count();

            // King + Knight/Bishop vs King
            if light_pieces_count == 1 {
                return true;
            }

            // King + Bishop (same color) vs King + Bishop (same color)
            if light_pieces_count == 2 {
                let white_bishops_bb = self.pieces[WHITE][BISHOP];
                let black_bishops_bb = self.pieces[BLACK][BISHOP];

                if white_bishops_bb != 0 && black_bishops_bb != 0 {
                    let all_bishops_bb = white_bishops_bb | black_bishops_bb;
                    if (all_bishops_bb & WHITE_SQUARES_BB) == all_bishops_bb || (all_bishops_bb & BLACK_SQUARES_BB) == all_bishops_bb {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Gets pieces count by counting set bits in occupancy.
    pub fn get_pieces_count(&self) -> u8 {
        (self.occupancy[WHITE] | self.occupancy[BLACK]).bit_count() as u8
    }

    /// Checks if there's an instant move possible and returns it as [Some], otherwise [None].
    pub fn get_instant_move(&mut self) -> Option<Move> {
        if !self.is_king_checked(self.stm) {
            return None;
        }

        let mut moves = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
        let moves_count = self.get_all_moves(&mut moves, u64::MAX);

        assert_fast!(moves_count < engine::MAX_MOVES_COUNT);

        let mut evading_moves_count = 0;
        let mut evading_move = Move::default();

        for r#move in &moves[0..moves_count] {
            let r#move = unsafe { r#move.assume_init() };
            self.make_move(r#move);

            if !self.is_king_checked(self.stm ^ 1) {
                evading_moves_count += 1;
                evading_move = r#move;

                if evading_moves_count > 1 {
                    self.undo_move(r#move);
                    return None;
                }
            }

            self.undo_move(r#move);
        }

        if evading_moves_count == 1 {
            return Some(evading_move);
        }

        None
    }

    /// Checks if there's a tablebase move (only Syzygy supported for now) and returns it as [Some], otherwise [None].
    pub fn get_tablebase_move(&self, probe_limit: u32) -> Option<(Move, i16)> {
        tablebases::get_tablebase_move(self, probe_limit)
    }

    /// Converts the board's state into FEN.
    pub fn to_fen(&self) -> String {
        fen::board_to_fen(self)
    }

    /// Converts the board`s state into EPD.
    pub fn to_epd(&self) -> String {
        fen::board_to_epd(self)
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            pieces: [[0; 6], [0; 6]],
            occupancy: [0; 2],
            piece_table: [u8::MAX; 64],
            stm: WHITE,
            null_moves: 0,
            game_phase: 0,
            fullmove_number: 1,
            state: BoardState {
                halfmove_clock: 0,
                castling_rights: CastlingRights::NONE,
                en_passant: 0,
                hash: 0,
                pawn_hash: 0,
                captured_piece: 0,
                pst_score: PackedEval::default(),
            },
            state_stack: Vec::new(),
            pawn_attacks: [0; 2],
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_fen())
    }
}

impl BoardState {
    /// Constructs a new instance of [BoardState] with stored `halfmove_clock`, `castling_rights`, `en_passant`, `hash`, `pawn_hash` and `captured_piece`.
    pub fn new(halfmove_clock: u16, castling_rights: u8, en_passant: u64, hash: u64, pawn_hash: u64, captured_piece: u8, pst_score: PackedEval) -> BoardState {
        BoardState { halfmove_clock, castling_rights, en_passant, hash, pawn_hash, captured_piece, pst_score }
    }
}
