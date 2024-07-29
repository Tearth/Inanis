use super::movegen::MagicContainer;
use super::movescan;
use super::movescan::Move;
use super::movescan::MoveFlags;
use super::patterns::PatternsContainer;
use super::text::fen;
use super::zobrist::ZobristContainer;
use super::*;
use crate::cache::pawns::PawnHashTable;
use crate::engine;
use crate::engine::see::SEEContainer;
use crate::engine::statistics::SearchStatistics;
use crate::evaluation::material;
use crate::evaluation::mobility;
use crate::evaluation::pawns;
use crate::evaluation::pst;
use crate::evaluation::safety;
use crate::evaluation::EvaluationParameters;
use crate::tablebases;
use crate::utils::bitflags::BitFlags;
use crate::utils::bithelpers::BitHelpers;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::MaybeUninit;
use std::sync::Arc;

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
    pub castling_rights: u8,
    pub en_passant: u64,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
    pub active_color: usize,
    pub hash: u64,
    pub pawn_hash: u64,
    pub null_moves: u8,
    pub captured_piece: usize,
    pub game_phase: u8,
    pub state_stack: Vec<BoardState>,
    pub material_scores: [i16; 2],
    pub pst_scores: [[i16; 2]; 2],
    pub pawn_attacks: [u64; 2],
    pub evaluation_parameters: Arc<EvaluationParameters>,
    pub zobrist: Arc<ZobristContainer>,
    pub patterns: Arc<PatternsContainer>,
    pub see: Arc<SEEContainer>,
    pub magic: Arc<MagicContainer>,
}

#[derive(Clone)]
pub struct BoardState {
    pub halfmove_clock: u16,
    pub castling_rights: u8,
    pub en_passant: u64,
    pub hash: u64,
    pub pawn_hash: u64,
    pub captured_piece: usize,
    pub pst_scores: [[i16; 2]; 2],
}

impl Board {
    /// Constructs a new instance of [Board], using provided containers. If the parameter is [None], then the new container is created.
    pub fn new(
        evaluation_parameters: Option<Arc<EvaluationParameters>>,
        zobrist_container: Option<Arc<ZobristContainer>>,
        patterns_container: Option<Arc<PatternsContainer>>,
        see_container: Option<Arc<SEEContainer>>,
        magic_container: Option<Arc<MagicContainer>>,
    ) -> Self {
        let evaluation_parameters = evaluation_parameters.unwrap_or_else(|| Arc::new(Default::default()));
        let zobrist_container = zobrist_container.unwrap_or_else(|| Arc::new(Default::default()));
        let patterns_container = patterns_container.unwrap_or_else(|| Arc::new(Default::default()));
        let see_container = see_container.unwrap_or_else(|| Arc::new(SEEContainer::new(Some(evaluation_parameters.clone()))));
        let magic_container = magic_container.unwrap_or_else(|| Arc::new(Default::default()));

        Board {
            pieces: [[0; 6], [0; 6]],
            occupancy: [0; 2],
            piece_table: [u8::MAX; 64],
            castling_rights: CastlingRights::NONE,
            en_passant: 0,
            halfmove_clock: 0,
            fullmove_number: 1,
            active_color: WHITE,
            hash: 0,
            pawn_hash: 0,
            null_moves: 0,
            captured_piece: 0,
            game_phase: 0,
            state_stack: Vec::new(),
            material_scores: [0; 2],
            pst_scores: [[0, 2]; 2],
            pawn_attacks: [0; 2],
            evaluation_parameters,
            zobrist: zobrist_container,
            patterns: patterns_container,
            see: see_container,
            magic: magic_container,
        }
    }

    /// Constructs a new instance of [Board] with initial position, using provided containers. If the parameter is [None], then the new container is created.
    pub fn new_initial_position(
        evaluation_parameters: Option<Arc<EvaluationParameters>>,
        zobrist_container: Option<Arc<ZobristContainer>>,
        patterns_container: Option<Arc<PatternsContainer>>,
        see_container: Option<Arc<SEEContainer>>,
        magic_container: Option<Arc<MagicContainer>>,
    ) -> Self {
        Board::new_from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            evaluation_parameters,
            zobrist_container,
            patterns_container,
            see_container,
            magic_container,
        )
        .unwrap()
    }

    /// Constructs a new instance of [Board] with position specified by `fen`, using provided containers. If the parameter is [None],
    /// then the new container is created. Returns [Err] with proper error message if `fen` couldn't be parsed correctly.
    pub fn new_from_fen(
        fen: &str,
        evaluation_parameters: Option<Arc<EvaluationParameters>>,
        zobrist_container: Option<Arc<ZobristContainer>>,
        patterns_container: Option<Arc<PatternsContainer>>,
        see_container: Option<Arc<SEEContainer>>,
        magic_container: Option<Arc<MagicContainer>>,
    ) -> Result<Self, String> {
        fen::fen_to_board(fen, evaluation_parameters, zobrist_container, patterns_container, see_container, magic_container)
    }

    /// Constructs a new instance of [Board] with position specified by list of `moves`, using provided containers. If the parameter is [None],
    /// then the new container is created. Returns [Err] with proper error message is `moves` couldn't be parsed correctly.
    pub fn new_from_moves(
        moves: &[&str],
        evaluation_parameters: Option<Arc<EvaluationParameters>>,
        zobrist_container: Option<Arc<ZobristContainer>>,
        patterns_container: Option<Arc<PatternsContainer>>,
        see_container: Option<Arc<SEEContainer>>,
        magic_container: Option<Arc<MagicContainer>>,
    ) -> Result<Self, String> {
        let mut board = Board::new_initial_position(evaluation_parameters, zobrist_container, patterns_container, see_container, magic_container);
        for premade_move in moves {
            let parsed_move = Move::from_long_notation(premade_move, &board)?;
            board.make_move(parsed_move);
        }

        Ok(board)
    }

    /// Generates all possible non-captures (if `CAPTURES` is false) or all possible captures (if `CAPTURES` is true) at the current position, stores
    /// them into `moves` list (starting from `index`) and returns index of the first free slot. Use `evasion_mask` with value different
    /// than `u64::MAX` to restrict generator to the specified squares (useful during checks).
    pub fn get_moves<const CAPTURES: bool>(&self, moves: &mut [MaybeUninit<Move>; engine::MAX_MOVES_COUNT], mut index: usize, evasion_mask: u64) -> usize {
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
    pub fn get_all_moves(&self, moves: &mut [MaybeUninit<Move>; engine::MAX_MOVES_COUNT], evasion_mask: u64) -> usize {
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
        self.push_state();

        let color = self.active_color;
        let enemy_color = self.active_color ^ 1;
        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(from);

        if self.en_passant != 0 {
            self.hash ^= self.zobrist.get_en_passant_hash(self.en_passant.bit_scan() & 7);
            self.en_passant = 0;
        }

        match flags {
            MoveFlags::SINGLE_PUSH => {
                self.move_piece::<false>(color, piece, from, to);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, to);

                if piece == PAWN {
                    self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, from);
                    self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, to);
                    self.recalculate_pawn_attacks(color);
                }
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece::<false>(color, piece, from, to);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, to);

                self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, to);

                let sign = (color as i8) * 2 - 1;
                self.en_passant = 1u64 << ((to as i8) + sign * 8);
                self.hash ^= self.zobrist.get_en_passant_hash(self.en_passant.bit_scan() & 7);

                self.recalculate_pawn_attacks(color);
            }
            MoveFlags::CAPTURE => {
                self.captured_piece = self.get_piece(to);
                self.remove_piece::<false>(enemy_color, self.captured_piece, to);
                self.hash ^= self.zobrist.get_piece_hash(enemy_color, self.captured_piece, to);

                if self.captured_piece == PAWN {
                    self.pawn_hash ^= self.zobrist.get_piece_hash(enemy_color, self.captured_piece, to);
                    self.recalculate_pawn_attacks(enemy_color);
                }

                self.move_piece::<false>(color, piece, from, to);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, to);

                if piece == PAWN {
                    self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, from);
                    self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, to);
                    self.recalculate_pawn_attacks(color);
                }
            }
            MoveFlags::SHORT_CASTLING => {
                let king_from = 3 + 56 * color;
                let king_to = 1 + 56 * color;

                self.move_piece::<false>(color, KING, king_from, king_to);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_from);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_to);

                let rook_from = 0 + 56 * color;
                let rook_to = 2 + 56 * color;

                self.move_piece::<false>(color, ROOK, rook_from, rook_to);
                self.hash ^= self.zobrist.get_piece_hash(color, ROOK, rook_from);
                self.hash ^= self.zobrist.get_piece_hash(color, ROOK, rook_to);
            }
            MoveFlags::LONG_CASTLING => {
                let king_from = 3 + 56 * color;
                let king_to = 5 + 56 * color;

                self.move_piece::<false>(color, KING, king_from, king_to);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_from);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_to);

                let rook_from = 7 + 56 * color;
                let rook_to = 4 + 56 * color;

                self.move_piece::<false>(color, ROOK, rook_from, rook_to);
                self.hash ^= self.zobrist.get_piece_hash(color, ROOK, rook_from);
                self.hash ^= self.zobrist.get_piece_hash(color, ROOK, rook_to);
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece::<false>(color, piece, from, to);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, to);

                self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, to);

                let sign = (color as isize) * 2 - 1;
                let enemy_pawn_square = ((to as isize) + sign * 8) as usize;

                self.remove_piece::<false>(enemy_color, PAWN, enemy_pawn_square);
                self.hash ^= self.zobrist.get_piece_hash(enemy_color, piece, enemy_pawn_square);
                self.pawn_hash ^= self.zobrist.get_piece_hash(enemy_color, piece, enemy_pawn_square);

                self.recalculate_pawn_attacks(color);
                self.recalculate_pawn_attacks(enemy_color);
            }
            _ => {
                let promotion_piece = r#move.get_promotion_piece();
                if flags.contains(MoveFlags::CAPTURE) {
                    self.captured_piece = self.get_piece(to);
                    self.remove_piece::<false>(enemy_color, self.captured_piece, to);
                    self.hash ^= self.zobrist.get_piece_hash(enemy_color, self.captured_piece, to);
                }

                self.remove_piece::<false>(color, PAWN, from);
                self.hash ^= self.zobrist.get_piece_hash(color, PAWN, from);
                self.pawn_hash ^= self.zobrist.get_piece_hash(color, PAWN, from);

                self.add_piece::<false>(color, promotion_piece, to);
                self.hash ^= self.zobrist.get_piece_hash(color, promotion_piece, to);

                self.recalculate_pawn_attacks(color);
            }
        }

        if piece == KING {
            self.castling_rights &= match color {
                WHITE => {
                    self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                    self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_LONG_CASTLING);

                    !CastlingRights::WHITE_CASTLING
                }
                BLACK => {
                    self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                    self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_LONG_CASTLING);

                    !CastlingRights::BLACK_CASTLING
                }
                _ => panic!("Invalid parameter: fen={}, color={}", self, color),
            };

            self.pawn_hash ^= self.zobrist.get_piece_hash(color, KING, from);
            self.pawn_hash ^= self.zobrist.get_piece_hash(color, KING, to);

            pst::recalculate_incremental_values(self);
        } else if piece == ROOK {
            match color {
                WHITE => match from {
                    A1 => {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
                        self.castling_rights &= !CastlingRights::WHITE_LONG_CASTLING;
                    }
                    H1 => {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                        self.castling_rights &= !CastlingRights::WHITE_SHORT_CASTLING;
                    }
                    _ => {}
                },
                BLACK => match from {
                    A8 => {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
                        self.castling_rights &= !CastlingRights::BLACK_LONG_CASTLING;
                    }
                    H8 => {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                        self.castling_rights &= !CastlingRights::BLACK_SHORT_CASTLING;
                    }
                    _ => {}
                },
                _ => panic!("Invalid parameter: fen={}, color={}", self, color),
            }
        }

        if self.captured_piece == ROOK {
            match enemy_color {
                WHITE => match to {
                    A1 => {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
                        self.castling_rights &= !CastlingRights::WHITE_LONG_CASTLING;
                    }
                    H1 => {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                        self.castling_rights &= !CastlingRights::WHITE_SHORT_CASTLING;
                    }
                    _ => {}
                },
                BLACK => match to {
                    A8 => {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
                        self.castling_rights &= !CastlingRights::BLACK_LONG_CASTLING;
                    }
                    H8 => {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                        self.castling_rights &= !CastlingRights::BLACK_SHORT_CASTLING;
                    }
                    _ => {}
                },
                _ => panic!("Invalid parameter: fen={}, color={}", self, color),
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

        self.hash ^= self.zobrist.get_active_color_hash();
        self.active_color = enemy_color;
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
        let color = self.active_color ^ 1;
        let enemy_color = self.active_color;
        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(to);

        match flags {
            MoveFlags::SINGLE_PUSH => {
                self.move_piece::<true>(color, piece, to, from);

                if piece == PAWN {
                    self.recalculate_pawn_attacks(color);
                }
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece::<true>(color, piece, to, from);
                self.recalculate_pawn_attacks(color);
            }
            MoveFlags::CAPTURE => {
                self.move_piece::<true>(color, piece, to, from);
                self.add_piece::<true>(enemy_color, self.captured_piece, to);

                if piece == PAWN {
                    self.recalculate_pawn_attacks(color);
                }

                if self.captured_piece == PAWN {
                    self.recalculate_pawn_attacks(enemy_color);
                }
            }
            MoveFlags::SHORT_CASTLING => {
                self.move_piece::<true>(color, KING, 1 + 56 * color, 3 + 56 * color);
                self.move_piece::<true>(color, ROOK, 2 + 56 * color, 0 + 56 * color);
            }
            MoveFlags::LONG_CASTLING => {
                self.move_piece::<true>(color, KING, 5 + 56 * color, 3 + 56 * color);
                self.move_piece::<true>(color, ROOK, 4 + 56 * color, 7 + 56 * color);
            }
            MoveFlags::EN_PASSANT => {
                let sign = (color as isize) * 2 - 1;
                let enemy_pawn_square = ((to as isize) + sign * 8) as usize;

                self.move_piece::<true>(color, piece, to, from);
                self.add_piece::<true>(enemy_color, PAWN, enemy_pawn_square);
                self.recalculate_pawn_attacks(color);
                self.recalculate_pawn_attacks(enemy_color);
            }
            _ => {
                self.add_piece::<true>(color, PAWN, from);
                self.remove_piece::<true>(color, piece, to);
                self.recalculate_pawn_attacks(color);

                if flags.contains(MoveFlags::CAPTURE) {
                    self.add_piece::<true>(enemy_color, self.captured_piece, to);
                }
            }
        }

        if color == BLACK {
            self.fullmove_number -= 1;
        }

        self.active_color = color;
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

        if self.en_passant != 0 {
            self.hash ^= self.zobrist.get_en_passant_hash(self.en_passant.bit_scan() & 7);
            self.en_passant = 0;
        }

        if self.active_color == BLACK {
            self.fullmove_number += 1;
        }

        self.null_moves += 1;
        self.active_color ^= 1;
        self.hash ^= self.zobrist.get_active_color_hash();
    }

    /// Undoes a null move, which is basically a switch of the active color with restoring of the internal state.
    ///
    /// Steps of undoing a null move:
    ///  - decrease fullmove number if needed
    ///  - switch active color
    ///  - decrease null moves count
    ///  - restore halfmove clock, castling rights, en passant bitboard, board hash and pawn hash
    pub fn undo_null_move(&mut self) {
        if self.active_color == WHITE {
            self.fullmove_number -= 1;
        }

        self.active_color ^= 1;
        self.null_moves -= 1;
        self.pop_state();
    }

    /// Preserves halfmove clock, castling rights, en passant bitboard, board hash, pawn hash and captured piece on the stack
    pub fn push_state(&mut self) {
        self.state_stack.push(BoardState::new(
            self.halfmove_clock,
            self.castling_rights,
            self.en_passant,
            self.hash,
            self.pawn_hash,
            self.captured_piece,
            self.pst_scores,
        ));
    }

    /// Restores halfmove clock, castling rights, en passant bitboard, board hash, pawn hash and captured piece from the stack
    pub fn pop_state(&mut self) {
        let state = self.state_stack.pop().unwrap();
        self.halfmove_clock = state.halfmove_clock;
        self.castling_rights = state.castling_rights;
        self.en_passant = state.en_passant;
        self.hash = state.hash;
        self.pawn_hash = state.pawn_hash;
        self.captured_piece = state.captured_piece;
        self.pst_scores = state.pst_scores;
    }

    /// Checks if the square specified by `square` is attacked by enemy, from the `color` perspective.
    pub fn is_square_attacked(&self, color: usize, square: usize) -> bool {
        let enemy_color = color ^ 1;
        let occupancy_bb = self.occupancy[WHITE] | self.occupancy[BLACK];

        let rook_queen_attacks_bb = self.magic.get_rook_moves(occupancy_bb, square);
        let enemy_rooks_queens_bb = self.pieces[enemy_color][ROOK] | self.pieces[enemy_color][QUEEN];
        if (rook_queen_attacks_bb & enemy_rooks_queens_bb) != 0 {
            return true;
        }

        let bishop_queen_attacks_bb = self.magic.get_bishop_moves(occupancy_bb, square);
        let enemy_bishops_queens_bb = self.pieces[enemy_color][BISHOP] | self.pieces[enemy_color][QUEEN];
        if (bishop_queen_attacks_bb & enemy_bishops_queens_bb) != 0 {
            return true;
        }

        let knight_attacks_bb = self.magic.get_knight_moves(square, &self.patterns);
        let enemy_knights_bb = self.pieces[enemy_color][KNIGHT];
        if (knight_attacks_bb & enemy_knights_bb) != 0 {
            return true;
        }

        let king_attacks_bb = self.magic.get_king_moves(square, &self.patterns);
        let enemy_kings_bb = self.pieces[enemy_color][KING];
        if (king_attacks_bb & enemy_kings_bb) != 0 {
            return true;
        }

        let square_bb = 1u64 << square;
        if (self.pawn_attacks[enemy_color] & square_bb) != 0 {
            return true;
        }

        false
    }

    /// Checks if any of the square specified by `squares` list is attacked by enemy, from the `color` perspective.
    pub fn are_squares_attacked(&self, color: usize, squares: &[usize]) -> bool {
        squares.iter().any(|square| self.is_square_attacked(color, *square))
    }

    /// Gets a list of enemy pieces attacking a square specified by `squares_index`, from the `color` perspective. The encoding looks as follows:
    ///  - bit 0 - Pawn
    ///  - bit 1, 2, 3 - Knight/Bishop
    ///  - bit 4, 5 - Rook
    ///  - bit 6 - Queen
    ///  - bit 7 - King
    pub fn get_attacking_pieces(&self, color: usize, square: usize) -> usize {
        let mut result = 0;
        let enemy_color = color ^ 1;
        let occupancy_bb = self.occupancy[WHITE] | self.occupancy[BLACK];

        let rooks_queens_bb = self.pieces[enemy_color][ROOK] | self.pieces[enemy_color][QUEEN];
        let bishops_queens_bb = self.pieces[enemy_color][BISHOP] | self.pieces[enemy_color][QUEEN];

        let king_attacks_bb = self.magic.get_king_moves(square, &self.patterns);
        let attacking_kings_count = ((king_attacks_bb & self.pieces[enemy_color][KING]) != 0) as usize;
        result |= attacking_kings_count << 7;

        let rook_attacks_bb = self.magic.get_rook_moves(occupancy_bb & !rooks_queens_bb, square);
        let attacking_rooks_count = (rook_attacks_bb & self.pieces[enemy_color][ROOK]).bit_count();
        let attacking_queens_count = ((rook_attacks_bb & self.pieces[enemy_color][QUEEN]) != 0) as usize;

        result |= match attacking_rooks_count {
            0 => 0,
            1 => 1 << 4,
            _ => 3 << 4,
        };
        result |= attacking_queens_count << 6;

        let knight_attacks_bb = self.magic.get_knight_moves(square, &self.patterns);
        let attacking_knights_count = (knight_attacks_bb & self.pieces[enemy_color][KNIGHT]).bit_count();

        let bishop_attacks_bb = self.magic.get_bishop_moves(occupancy_bb & !bishops_queens_bb, square);
        let attacking_bishops_count = (bishop_attacks_bb & self.pieces[enemy_color][BISHOP]).bit_count();
        let attacking_knights_bishops_count = attacking_knights_count + attacking_bishops_count;
        let attacking_queens_count = ((bishop_attacks_bb & self.pieces[enemy_color][QUEEN]) != 0) as usize;

        result |= match attacking_knights_bishops_count {
            0 => 0,
            1 => 1 << 1,
            2 => 3 << 1,
            _ => 7 << 1,
        };
        result |= attacking_queens_count << 6;

        let square_bb = 1u64 << square;
        let attacking_pawns_count = ((self.pawn_attacks[enemy_color] & square_bb) != 0) as usize;

        result |= attacking_pawns_count;
        result
    }

    /// Check if the king of the `color` side is checked.
    pub fn is_king_checked(&self, color: usize) -> bool {
        if self.pieces[color][KING] == 0 {
            return false;
        }

        self.is_square_attacked(color, (self.pieces[color][KING]).bit_scan())
    }

    /// Gets piece on the square specified by `square`.
    pub fn get_piece(&self, square: usize) -> usize {
        let piece = self.piece_table[square];
        if piece == u8::MAX {
            return usize::MAX;
        }

        piece as usize
    }

    /// Gets piece's color on the square specified by `square`. Returns `u8::MAX` if there is no piece there.
    pub fn get_piece_color(&self, square: usize) -> usize {
        let piece = self.piece_table[square];
        if piece == u8::MAX {
            return usize::MAX;
        }

        (((1u64 << square) & self.occupancy[WHITE]) == 0) as usize
    }

    /// Adds `piece` on the `square` with the specified `color`, also updates occupancy and incremental values.
    pub fn add_piece<const UNDO: bool>(&mut self, color: usize, piece: usize, square: usize) {
        self.pieces[color][piece] |= 1u64 << square;
        self.occupancy[color] |= 1u64 << square;
        self.piece_table[square] = piece as u8;
        self.material_scores[color] += self.evaluation_parameters.piece_value[piece];
        self.game_phase += self.evaluation_parameters.piece_phase_value[piece];

        if !UNDO {
            let king_file = self.pieces[color][KING].bit_scan() & 7;
            self.pst_scores[color][OPENING] += self.evaluation_parameters.get_pst_value(color, piece, king_file, OPENING, square);
            self.pst_scores[color][ENDING] += self.evaluation_parameters.get_pst_value(color, piece, king_file, ENDING, square);
        }
    }

    /// Removes `piece` on the `square` with the specified `color`, also updates occupancy and incremental values.
    pub fn remove_piece<const UNDO: bool>(&mut self, color: usize, piece: usize, square: usize) {
        self.pieces[color][piece] &= !(1u64 << square);
        self.occupancy[color] &= !(1u64 << square);
        self.piece_table[square] = u8::MAX;
        self.material_scores[color] -= self.evaluation_parameters.piece_value[piece];
        self.game_phase -= self.evaluation_parameters.piece_phase_value[piece];

        if !UNDO {
            let king_file = self.pieces[color][KING].bit_scan() & 7;
            self.pst_scores[color][OPENING] -= self.evaluation_parameters.get_pst_value(color, piece, king_file, OPENING, square);
            self.pst_scores[color][ENDING] -= self.evaluation_parameters.get_pst_value(color, piece, king_file, ENDING, square);
        }
    }

    /// Moves `piece` from the square specified by `from` to the square specified by `to` with the specified `color`, also updates occupancy and incremental values.
    pub fn move_piece<const UNDO: bool>(&mut self, color: usize, piece: usize, from: usize, to: usize) {
        self.pieces[color][piece] ^= (1u64 << from) | (1u64 << to);
        self.occupancy[color] ^= (1u64 << from) | (1u64 << to);

        self.piece_table[to] = self.piece_table[from];
        self.piece_table[from] = u8::MAX;

        if !UNDO {
            let king_file = self.pieces[color][KING].bit_scan() & 7;
            self.pst_scores[color][OPENING] -= self.evaluation_parameters.get_pst_value(color, piece, king_file, OPENING, from);
            self.pst_scores[color][ENDING] -= self.evaluation_parameters.get_pst_value(color, piece, king_file, ENDING, from);
            self.pst_scores[color][OPENING] += self.evaluation_parameters.get_pst_value(color, piece, king_file, OPENING, to);
            self.pst_scores[color][ENDING] += self.evaluation_parameters.get_pst_value(color, piece, king_file, ENDING, to);
        }
    }

    /// Recalculates board's hashes entirely.
    pub fn recalculate_hashes(&mut self) {
        zobrist::recalculate_hash(self);
        zobrist::recalculate_pawn_hash(self);
    }

    /// Recalculate pawn attacks for the specific `color`.
    pub fn recalculate_pawn_attacks(&mut self, color: usize) {
        let pawns_bb = self.pieces[color][PAWN];
        self.pawn_attacks[color] = match color {
            WHITE => ((pawns_bb & !FILE_A_BB) << 9) | ((pawns_bb & !FILE_H_BB) << 7),
            BLACK => ((pawns_bb & !FILE_A_BB) >> 7) | ((pawns_bb & !FILE_H_BB) >> 9),
            _ => {
                panic!("Invalid value: color={}", color);
            }
        };
    }

    /// Runs full evaluation (material, piece-square tables, mobility, pawns structure and safety) of the current position, using `pawn_hashtable` to store pawn
    /// evaluations and `statistics` to gather diagnostic data. Returns score from the `color` perspective (more than 0 when advantage, less than 0 when disadvantage).
    pub fn evaluate<const DIAG: bool>(&self, color: usize, pawn_hashtable: &PawnHashTable, statistics: &mut SearchStatistics) -> i16 {
        let mut dangered_white_king_squares = 0;
        let mut dangered_black_king_squares = 0;
        let mobility_score = mobility::evaluate(self, &mut dangered_white_king_squares, &mut dangered_black_king_squares);

        let game_phase = self.game_phase;
        let initial_game_phase = self.evaluation_parameters.initial_game_phase;
        let evaluation = 0
            + material::evaluate(self)
            + pst::evaluate(self)
            + pawns::evaluate::<DIAG>(self, pawn_hashtable, statistics)
            + safety::evaluate(self, dangered_white_king_squares, dangered_black_king_squares)
            + mobility_score;
        let sign = -((color as i16) * 2 - 1);

        sign * evaluation.taper_score(game_phase, initial_game_phase)
    }

    /// Runs full evaluation (material, piece-square tables, mobility, pawns structure and safety) of the current position.
    /// Returns score from the `color` perspective (more than 0 when advantage, less than 0 when disadvantage).
    pub fn evaluate_without_cache(&self, color: usize) -> i16 {
        let mut dangered_white_king_squares = 0;
        let mut dangered_black_king_squares = 0;
        let mobility_score = mobility::evaluate(self, &mut dangered_white_king_squares, &mut dangered_black_king_squares);

        let game_phase = self.game_phase;
        let initial_game_phase = self.evaluation_parameters.initial_game_phase;
        let evaluation = 0
            + material::evaluate(self)
            + pst::evaluate(self)
            + pawns::evaluate_without_cache(self)
            + safety::evaluate(self, dangered_white_king_squares, dangered_black_king_squares)
            + mobility_score;
        let sign = -((color as i16) * 2 - 1);

        sign * evaluation.taper_score(game_phase, initial_game_phase)
    }

    /// Runs lazy (fast) evaluations, considering only material and piece-square tables. Returns score from the `color` perspective (more than 0 when
    /// advantage, less than 0 when disadvantage).
    pub fn evaluate_lazy(&self, color: usize) -> i16 {
        let game_phase = self.game_phase;
        let initial_game_phase = self.evaluation_parameters.initial_game_phase;
        let evaluation = material::evaluate(self) + pst::evaluate(self);
        let sign = -((color as i16) * 2 - 1);

        sign * evaluation.taper_score(game_phase, initial_game_phase)
    }

    /// Recalculates incremental values (material and piece-square tables) entirely.
    pub fn recalculate_incremental_values(&mut self) {
        material::recalculate_incremental_values(self);
        pst::recalculate_incremental_values(self);
    }

    /// Checks if there's repetition draw with the specified `threshold` (should be 3 in the most cases) at the current position.
    pub fn is_repetition_draw(&self, threshold: i32) -> bool {
        if self.state_stack.len() < 6 || self.null_moves > 0 {
            return false;
        }

        let mut repetitions_count = 1;
        let from = self.state_stack.len().saturating_sub(self.halfmove_clock as usize);
        let to = self.state_stack.len() - 1;

        for hash_index in (from..to).rev().step_by(2) {
            if self.state_stack[hash_index].hash == self.hash {
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

        self.halfmove_clock >= 100
    }

    /// Checks if there's an inssuficient material draw:
    ///  - King vs King
    ///  - King + Knight/Bishop vs King
    ///  - King + Bishop (same color) vs King + Bishop (same color)
    pub fn is_insufficient_material_draw(&self) -> bool {
        let white_material = self.material_scores[WHITE] - self.evaluation_parameters.piece_value[KING];
        let black_material = self.material_scores[BLACK] - self.evaluation_parameters.piece_value[KING];
        let bishop_value = self.evaluation_parameters.piece_value[BISHOP];

        let pawns_bb = self.pieces[WHITE][PAWN] | self.pieces[BLACK][PAWN];
        let pawns_count = pawns_bb.bit_count();

        if white_material <= bishop_value && black_material <= bishop_value && pawns_count == 0 {
            // King vs King
            if white_material == 0 && black_material == 0 {
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
        if !self.is_king_checked(self.active_color) {
            return None;
        }

        let mut moves = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
        let moves_count = self.get_all_moves(&mut moves, u64::MAX);

        let mut evading_moves_count = 0;
        let mut evading_move = Default::default();

        for r#move in &moves[0..moves_count] {
            let r#move = unsafe { r#move.assume_init() };
            self.make_move(r#move);

            if !self.is_king_checked(self.active_color ^ 1) {
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

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_fen())
    }
}

impl BoardState {
    /// Constructs a new instance of [BoardState] with stored `halfmove_clock`, `castling_rights`, `en_passant`, `hash`, `pawn_hash` and `captured_piece`.
    pub fn new(
        halfmove_clock: u16,
        castling_rights: u8,
        en_passant: u64,
        hash: u64,
        pawn_hash: u64,
        captured_piece: usize,
        pst_scores: [[i16; 2]; 2],
    ) -> BoardState {
        BoardState { halfmove_clock, castling_rights, en_passant, hash, pawn_hash, captured_piece, pst_scores }
    }
}
