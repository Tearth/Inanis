use super::movegen::MagicContainer;
use super::movescan;
use super::movescan::Move;
use super::movescan::MoveFlags;
use super::patterns::PatternsContainer;
use super::zobrist::ZobristContainer;
use super::*;
use crate::cache::pawns::PawnHashTable;
use crate::engine;
use crate::engine::context::SearchStatistics;
use crate::engine::see::SEEContainer;
use crate::evaluation::material;
use crate::evaluation::mobility;
use crate::evaluation::pawns;
use crate::evaluation::pst;
use crate::evaluation::safety;
use crate::evaluation::EvaluationParameters;
use crate::utils::bitflags::BitFlags;
use crate::utils::fen;
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
    pub active_color: u8,
    pub hash: u64,
    pub pawn_hash: u64,
    pub null_moves: u8,
    pub captured_piece: u8,
    pub game_phase: u8,
    pub state_stack: Vec<BitboardState>,
    pub material_scores: [i16; 2],
    pub pst_scores: [[i16; 2]; 2],
    pub evaluation_parameters: Arc<EvaluationParameters>,
    pub zobrist: Arc<ZobristContainer>,
    pub patterns: Arc<PatternsContainer>,
    pub see: Arc<SEEContainer>,
    pub magic: Arc<MagicContainer>,
}

#[derive(Clone)]
pub struct BitboardState {
    pub halfmove_clock: u16,
    pub castling_rights: u8,
    pub en_passant: u64,
    pub hash: u64,
    pub pawn_hash: u64,
    pub captured_piece: u8,
}

impl Board {
    /// Constructs a new instance of [Bitboard], using provided containers. If the parameter is [None], then the new container is created.
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
            evaluation_parameters,
            zobrist: zobrist_container,
            patterns: patterns_container,
            see: see_container,
            magic: magic_container,
        }
    }

    /// Constructs a new instance of [Bitboard] with initial position, using provided containers. If the parameter is [None], then the new container is created.
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

    /// Constructs a new instance of [Bitboard] with position specified by `fen`, using provided containers. If the parameter is [None],
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

    /// Constructs a new instance of [Bitboard] with position specified by list of `moves`, using provided containers. If the parameter is [None],
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
            self.hash ^= self.zobrist.get_en_passant_hash((bit_scan(self.en_passant) & 7) as u8);
            self.en_passant = 0;
        }

        match flags {
            MoveFlags::SINGLE_PUSH => {
                self.move_piece(color, piece, from, to);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, to);

                if piece == PAWN {
                    self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, from);
                    self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, to);
                }
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(color, piece, from, to);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, to);

                self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, to);

                self.en_passant = 1u64 << ((to as i8) + 8 * ((color as i8) * 2 - 1));
                self.hash ^= self.zobrist.get_en_passant_hash((bit_scan(self.en_passant) & 7) as u8);
            }
            MoveFlags::CAPTURE => {
                self.captured_piece = self.get_piece(to);
                self.remove_piece(enemy_color, self.captured_piece, to);
                self.hash ^= self.zobrist.get_piece_hash(enemy_color, self.captured_piece, to);

                if self.captured_piece == PAWN {
                    self.pawn_hash ^= self.zobrist.get_piece_hash(enemy_color, self.captured_piece, to);
                }

                self.move_piece(color, piece, from, to);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, to);

                if piece == PAWN {
                    self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, from);
                    self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, to);
                }
            }
            MoveFlags::SHORT_CASTLING => {
                let king_from = 3 + 56 * color;
                let king_to = 1 + 56 * color;

                self.move_piece(color, KING, king_from, king_to);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_from);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_to);

                let rook_from = 0 + 56 * color;
                let rook_to = 2 + 56 * color;

                self.move_piece(color, ROOK, rook_from, rook_to);
                self.hash ^= self.zobrist.get_piece_hash(color, ROOK, rook_from);
                self.hash ^= self.zobrist.get_piece_hash(color, ROOK, rook_to);
            }
            MoveFlags::LONG_CASTLING => {
                let king_from = 3 + 56 * color;
                let king_to = 5 + 56 * color;

                self.move_piece(color, KING, king_from, king_to);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_from);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_to);

                let rook_from = 7 + 56 * color;
                let rook_to = 4 + 56 * color;

                self.move_piece(color, ROOK, rook_from, rook_to);
                self.hash ^= self.zobrist.get_piece_hash(color, ROOK, rook_from);
                self.hash ^= self.zobrist.get_piece_hash(color, ROOK, rook_to);
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece(color, piece, from, to);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.hash ^= self.zobrist.get_piece_hash(color, piece, to);

                self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, from);
                self.pawn_hash ^= self.zobrist.get_piece_hash(color, piece, to);

                let enemy_pawn_square_index = ((to as i8) + 8 * ((color as i8) * 2 - 1)) as u8;

                self.remove_piece(enemy_color, PAWN, enemy_pawn_square_index);
                self.hash ^= self.zobrist.get_piece_hash(enemy_color, piece, enemy_pawn_square_index);
                self.pawn_hash ^= self.zobrist.get_piece_hash(enemy_color, piece, enemy_pawn_square_index);
            }
            _ => {
                let promotion_piece = r#move.get_promotion_piece();
                if flags.contains(MoveFlags::CAPTURE) {
                    self.captured_piece = self.get_piece(to);
                    self.remove_piece(enemy_color, self.captured_piece, to);
                    self.hash ^= self.zobrist.get_piece_hash(enemy_color, self.captured_piece, to);
                }

                self.remove_piece(color, PAWN, from);
                self.hash ^= self.zobrist.get_piece_hash(color, PAWN, from);
                self.pawn_hash ^= self.zobrist.get_piece_hash(color, PAWN, from);

                self.add_piece(color, promotion_piece, to);
                self.hash ^= self.zobrist.get_piece_hash(color, promotion_piece, to);
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
                _ => panic!("Invalid parameter: fen={}, color={}", self.to_fen(), color),
            };

            self.pawn_hash ^= self.zobrist.get_piece_hash(color, KING, from);
            self.pawn_hash ^= self.zobrist.get_piece_hash(color, KING, to);
        } else if piece == ROOK {
            match color {
                WHITE => {
                    if from == 0 {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
                        self.castling_rights &= !CastlingRights::WHITE_SHORT_CASTLING;
                    } else if from == 7 {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
                        self.castling_rights &= !CastlingRights::WHITE_LONG_CASTLING;
                    }
                }
                BLACK => {
                    if from == 56 {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
                        self.castling_rights &= !CastlingRights::BLACK_SHORT_CASTLING;
                    } else if from == 63 {
                        self.hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
                        self.castling_rights &= !CastlingRights::BLACK_LONG_CASTLING;
                    }
                }
                _ => panic!("Invalid parameter: fen={}, color={}", self.to_fen(), color),
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
                self.move_piece(color, piece, to, from);
            }
            MoveFlags::DOUBLE_PUSH => {
                self.move_piece(color, piece, to, from);
            }
            MoveFlags::CAPTURE => {
                self.move_piece(color, piece, to, from);
                self.add_piece(enemy_color, self.captured_piece, to);
            }
            MoveFlags::SHORT_CASTLING => {
                self.move_piece(color, KING, 1 + 56 * color, 3 + 56 * color);
                self.move_piece(color, ROOK, 2 + 56 * color, 0 + 56 * color);
            }
            MoveFlags::LONG_CASTLING => {
                self.move_piece(color, KING, 5 + 56 * color, 3 + 56 * color);
                self.move_piece(color, ROOK, 4 + 56 * color, 7 + 56 * color);
            }
            MoveFlags::EN_PASSANT => {
                self.move_piece(color, piece, to, from);
                self.add_piece(enemy_color, PAWN, ((to as i8) + 8 * ((color as i8) * 2 - 1)) as u8);
            }
            _ => {
                self.add_piece(color, PAWN, from);
                self.remove_piece(color, piece, to);

                if flags.contains(MoveFlags::CAPTURE) {
                    self.add_piece(enemy_color, self.captured_piece, to);
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
            self.hash ^= self.zobrist.get_en_passant_hash((bit_scan(self.en_passant) & 7) as u8);
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
        self.state_stack.push(BitboardState::new(self.halfmove_clock, self.castling_rights, self.en_passant, self.hash, self.pawn_hash, self.captured_piece));
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
    }

    /// Checks if the square specified by `square_index` is attacked by enemy, from the `color` perspective.
    pub fn is_square_attacked(&self, color: u8, square_index: u8) -> bool {
        let enemy_color = color ^ 1;
        let occupancy = self.occupancy[WHITE as usize] | self.occupancy[BLACK as usize];

        let rook_queen_attacks = self.magic.get_rook_moves(occupancy, square_index as usize);
        let enemy_rooks_queens = self.pieces[enemy_color as usize][ROOK as usize] | self.pieces[enemy_color as usize][QUEEN as usize];
        if (rook_queen_attacks & enemy_rooks_queens) != 0 {
            return true;
        }

        let bishop_queen_attacks = self.magic.get_bishop_moves(occupancy, square_index as usize);
        let enemy_bishops_queens = self.pieces[enemy_color as usize][BISHOP as usize] | self.pieces[enemy_color as usize][QUEEN as usize];
        if (bishop_queen_attacks & enemy_bishops_queens) != 0 {
            return true;
        }

        let knight_attacks = self.magic.get_knight_moves(square_index as usize, &self.patterns);
        let enemy_knights = self.pieces[enemy_color as usize][KNIGHT as usize];
        if (knight_attacks & enemy_knights) != 0 {
            return true;
        }

        let king_attacks = self.magic.get_king_moves(square_index as usize, &self.patterns);
        let enemy_kings = self.pieces[enemy_color as usize][KING as usize];
        if (king_attacks & enemy_kings) != 0 {
            return true;
        }

        let square = 1u64 << square_index;
        let potential_enemy_pawns = king_attacks & self.pieces[enemy_color as usize][PAWN as usize];
        let attacking_enemy_pawns = match color {
            WHITE => square & ((potential_enemy_pawns >> 7) | (potential_enemy_pawns >> 9)),
            BLACK => square & ((potential_enemy_pawns << 7) | (potential_enemy_pawns << 9)),
            _ => panic!("Invalid parameter: fen={}, color={}", self.to_fen(), color),
        };

        if attacking_enemy_pawns != 0 {
            return true;
        }

        false
    }

    /// Checks if any of the square specified by `square_indexes` list is attacked by enemy, from the `color` perspective.
    pub fn are_squares_attacked(&self, color: u8, square_indexes: &[u8]) -> bool {
        square_indexes.iter().any(|square_index| self.is_square_attacked(color, *square_index))
    }

    /// Gets a list of enemy pieces attacking a square specified by `squares_index`, from the `color` perspective. The encoding looks as follows:
    ///  - bit 0 - Pawn
    ///  - bit 1, 2, 3 - Knight/Bishop
    ///  - bit 4, 5 - Rook
    ///  - bit 6 - Queen
    ///  - bit 7 - King
    pub fn get_attacking_pieces(&self, color: u8, square_index: u8) -> u8 {
        let mut result = 0;
        let enemy_color = color ^ 1;
        let occupancy = self.occupancy[WHITE as usize] | self.occupancy[BLACK as usize];

        let bishops_rooks = self.pieces[enemy_color as usize][BISHOP as usize] | self.pieces[enemy_color as usize][ROOK as usize];
        let rooks_queens = self.pieces[enemy_color as usize][ROOK as usize] | self.pieces[enemy_color as usize][QUEEN as usize];
        let bishops_queens = self.pieces[enemy_color as usize][BISHOP as usize] | self.pieces[enemy_color as usize][QUEEN as usize];

        let king_attacks = self.magic.get_king_moves(square_index as usize, &self.patterns);
        let attacking_kings_count = ((king_attacks & self.pieces[enemy_color as usize][KING as usize]) != 0) as u8;
        result |= attacking_kings_count << 7;

        let queen_attacks = self.magic.get_queen_moves(occupancy & !bishops_rooks, square_index as usize);
        let attacking_queens_count = ((queen_attacks & self.pieces[enemy_color as usize][QUEEN as usize]) != 0) as u8;
        result |= attacking_queens_count << 6;

        let rook_attacks = self.magic.get_rook_moves(occupancy & !rooks_queens, square_index as usize);
        let attacking_rooks_count = bit_count(rook_attacks & self.pieces[enemy_color as usize][ROOK as usize]);

        result |= match attacking_rooks_count {
            0 => 0,
            1 => 1 << 4,
            _ => 3 << 4,
        };

        let knight_attacks = self.magic.get_knight_moves(square_index as usize, &self.patterns);
        let attacking_knights_count = bit_count(knight_attacks & self.pieces[enemy_color as usize][KNIGHT as usize]);
        let bishop_attacks = self.magic.get_bishop_moves(occupancy & !bishops_queens, square_index as usize);
        let attacking_bishops_count = bit_count(bishop_attacks & self.pieces[enemy_color as usize][BISHOP as usize]);
        let attacking_knights_bishops_count = attacking_knights_count + attacking_bishops_count;

        result |= match attacking_knights_bishops_count {
            0 => 0,
            1 => 1 << 1,
            2 => 3 << 1,
            _ => 7 << 1,
        };

        let square = 1u64 << square_index;
        let potential_enemy_pawns = king_attacks & self.pieces[enemy_color as usize][PAWN as usize];
        let attacking_pawns_count = (match color {
            WHITE => square & ((potential_enemy_pawns >> 7) | (potential_enemy_pawns >> 9)),
            BLACK => square & ((potential_enemy_pawns << 7) | (potential_enemy_pawns << 9)),
            _ => panic!("Invalid parameter: fen={}, color={}", self.to_fen(), color),
        } != 0) as u8;

        result |= attacking_pawns_count;
        result
    }

    /// Check if the king of the `color` side is checked.
    pub fn is_king_checked(&self, color: u8) -> bool {
        if self.pieces[color as usize][KING as usize] == 0 {
            return false;
        }

        self.is_square_attacked(color, bit_scan(self.pieces[color as usize][KING as usize]))
    }

    /// Gets piece on the square specified by `square_index`.
    pub fn get_piece(&self, square_index: u8) -> u8 {
        self.piece_table[square_index as usize]
    }

    /// Gets piece's color on the square specified by `square_index`. Returns `u8::MAX` if there is no piece there.
    pub fn get_piece_color(&self, square_index: u8) -> u8 {
        let piece = self.piece_table[square_index as usize];
        if piece == u8::MAX {
            return u8::MAX;
        }

        if ((1u64 << square_index) & self.occupancy[WHITE as usize]) != 0 {
            WHITE
        } else {
            BLACK
        }
    }

    /// Adds `piece` on the `square` with the specified `color`, also updates occupancy and incremental values.
    pub fn add_piece(&mut self, color: u8, piece: u8, square: u8) {
        self.pieces[color as usize][piece as usize] |= 1u64 << square;
        self.occupancy[color as usize] |= 1u64 << square;
        self.piece_table[square as usize] = piece;
        self.material_scores[color as usize] += self.evaluation_parameters.piece_value[piece as usize];
        self.game_phase += self.evaluation_parameters.piece_phase_value[piece as usize];

        self.pst_scores[color as usize][OPENING as usize] += self.evaluation_parameters.get_pst_value(color, piece, OPENING, square);
        self.pst_scores[color as usize][ENDING as usize] += self.evaluation_parameters.get_pst_value(color, piece, ENDING, square);
    }

    /// Removes `piece` on the `square` with the specified `color`, also updates occupancy and incremental values.
    pub fn remove_piece(&mut self, color: u8, piece: u8, square: u8) {
        self.pieces[color as usize][piece as usize] &= !(1u64 << square);
        self.occupancy[color as usize] &= !(1u64 << square);
        self.piece_table[square as usize] = u8::MAX;
        self.material_scores[color as usize] -= self.evaluation_parameters.piece_value[piece as usize];
        self.game_phase -= self.evaluation_parameters.piece_phase_value[piece as usize];

        self.pst_scores[color as usize][OPENING as usize] -= self.evaluation_parameters.get_pst_value(color, piece, OPENING, square);
        self.pst_scores[color as usize][ENDING as usize] -= self.evaluation_parameters.get_pst_value(color, piece, ENDING, square);
    }

    /// Moves `piece` from the square specified by `from` to the square specified by `to` with the specified `color`, also updates occupancy and incremental values.
    pub fn move_piece(&mut self, color: u8, piece: u8, from: u8, to: u8) {
        self.pieces[color as usize][piece as usize] ^= (1u64 << from) | (1u64 << to);
        self.occupancy[color as usize] ^= (1u64 << from) | (1u64 << to);

        self.piece_table[to as usize] = self.piece_table[from as usize];
        self.piece_table[from as usize] = u8::MAX;

        self.pst_scores[color as usize][OPENING as usize] -= self.evaluation_parameters.get_pst_value(color, piece, OPENING, from);
        self.pst_scores[color as usize][ENDING as usize] -= self.evaluation_parameters.get_pst_value(color, piece, ENDING, from);
        self.pst_scores[color as usize][OPENING as usize] += self.evaluation_parameters.get_pst_value(color, piece, OPENING, to);
        self.pst_scores[color as usize][ENDING as usize] += self.evaluation_parameters.get_pst_value(color, piece, ENDING, to);
    }

    /// Converts the board's state into FEN.
    pub fn to_fen(&self) -> String {
        fen::board_to_fen(self)
    }

    /// Converts the board`s state into EPD.
    pub fn to_epd(&self) -> String {
        fen::board_to_epd(self)
    }

    /// Recalculates board's hash entirely.
    pub fn recalculate_hash(&mut self) {
        let mut hash = 0u64;

        for color in 0..2 {
            for piece_index in 0..6 {
                let mut pieces = self.pieces[color as usize][piece_index as usize];
                while pieces != 0 {
                    let square = get_lsb(pieces);
                    let square_index = bit_scan(square);
                    pieces = pop_lsb(pieces);

                    hash ^= self.zobrist.get_piece_hash(color, piece_index, square_index);
                }
            }
        }

        if self.castling_rights.contains(CastlingRights::WHITE_SHORT_CASTLING) {
            hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_SHORT_CASTLING);
        }
        if self.castling_rights.contains(CastlingRights::WHITE_LONG_CASTLING) {
            hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::WHITE_LONG_CASTLING);
        }
        if self.castling_rights.contains(CastlingRights::BLACK_SHORT_CASTLING) {
            hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_SHORT_CASTLING);
        }
        if self.castling_rights.contains(CastlingRights::BLACK_LONG_CASTLING) {
            hash ^= self.zobrist.get_castling_right_hash(self.castling_rights, CastlingRights::BLACK_LONG_CASTLING);
        }

        if self.en_passant != 0 {
            hash ^= self.zobrist.get_en_passant_hash((bit_scan(self.en_passant) & 7) as u8);
        }

        if self.active_color == BLACK {
            hash ^= self.zobrist.get_active_color_hash();
        }

        self.hash = hash;
    }

    /// Recalculates board's pawn hash entirely.
    pub fn recalculate_pawn_hash(&mut self) {
        let mut hash = 0u64;

        for color in 0..2 {
            for piece in [PAWN, KING] {
                let mut pieces = self.pieces[color as usize][piece as usize];
                while pieces != 0 {
                    let square = get_lsb(pieces);
                    let square_index = bit_scan(square);
                    pieces = pop_lsb(pieces);

                    hash ^= self.zobrist.get_piece_hash(color, piece, square_index);
                }
            }
        }

        self.pawn_hash = hash;
    }

    /// Runs full evaluation (material, piece-square tables, mobility, pawns structure and safety) of the current position, using `pawn_hashtable` to store pawn
    /// evaluations and `statistics` to gather diagnostic data. Returns score from the `color` perspective (more than 0 when advantage, less than 0 when disadvantage).
    pub fn evaluate<const DIAG: bool>(&self, color: u8, pawn_hashtable: &PawnHashTable, statistics: &mut SearchStatistics) -> i16 {
        let mut white_attack_mask = 0;
        let mut black_attack_mask = 0;
        let mobility_score = mobility::evaluate(self, &mut white_attack_mask, &mut black_attack_mask);

        let game_phase = self.game_phase;
        let initial_game_phase = self.evaluation_parameters.initial_game_phase;

        let evaluation = 0
            + material::evaluate(self)
            + pst::evaluate(self)
            + pawns::evaluate::<DIAG>(self, pawn_hashtable, statistics)
            + safety::evaluate(self, white_attack_mask, black_attack_mask)
            + mobility_score;

        -((color as i16) * 2 - 1) * evaluation.taper_score(game_phase, initial_game_phase)
    }

    /// Runs full evaluation (material, piece-square tables, mobility, pawns structure and safety) of the current position.
    /// Returns score from the `color` perspective (more than 0 when advantage, less than 0 when disadvantage).
    pub fn evaluate_without_cache(&self, color: u8) -> i16 {
        let mut white_attack_mask = 0;
        let mut black_attack_mask = 0;
        let mobility_score = mobility::evaluate(self, &mut white_attack_mask, &mut black_attack_mask);

        let game_phase = self.game_phase;
        let initial_game_phase = self.evaluation_parameters.initial_game_phase;

        let evaluation = 0
            + material::evaluate(self)
            + pst::evaluate(self)
            + pawns::evaluate_without_cache(self)
            + safety::evaluate(self, white_attack_mask, black_attack_mask)
            + mobility_score;

        -((color as i16) * 2 - 1) * evaluation.taper_score(game_phase, initial_game_phase)
    }

    /// Runs lazy (fast) evaluations, considering only material and piece-square tables. Returns score from the `color` perspective (more than 0 when
    /// advantage, less than 0 when disadvantage).
    pub fn evaluate_lazy(&self, color: u8) -> i16 {
        let game_phase = self.game_phase;
        let initial_game_phase = self.evaluation_parameters.initial_game_phase;

        -((color as i16) * 2 - 1) * (material::evaluate(self) + pst::evaluate(self)).taper_score(game_phase, initial_game_phase)
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
        let mut from = self.state_stack.len().wrapping_sub(self.halfmove_clock as usize);
        let to = self.state_stack.len() - 1;

        if from > 1024 {
            from = 0;
        }

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
        let white_material = self.material_scores[WHITE as usize] - self.evaluation_parameters.piece_value[KING as usize];
        let black_material = self.material_scores[BLACK as usize] - self.evaluation_parameters.piece_value[KING as usize];
        let bishop_value = self.evaluation_parameters.piece_value[BISHOP as usize];
        let pawns_count = bit_count(self.pieces[WHITE as usize][PAWN as usize]) + bit_count(self.pieces[BLACK as usize][PAWN as usize]);

        if white_material <= bishop_value && black_material <= bishop_value && pawns_count == 0 {
            // King vs King
            if white_material == 0 && black_material == 0 {
                return true;
            }

            let light_pieces_count = 0
                + bit_count(self.pieces[WHITE as usize][KNIGHT as usize])
                + bit_count(self.pieces[WHITE as usize][BISHOP as usize])
                + bit_count(self.pieces[BLACK as usize][KNIGHT as usize])
                + bit_count(self.pieces[BLACK as usize][BISHOP as usize]);

            // King + Knight/Bishop vs King
            if light_pieces_count == 1 {
                return true;
            }

            // King + Bishop (same color) vs King + Bishop (same color)
            if light_pieces_count == 2 {
                let white_bishops = self.pieces[WHITE as usize][BISHOP as usize];
                let black_bishops = self.pieces[BLACK as usize][BISHOP as usize];

                if white_bishops != 0 && black_bishops != 0 {
                    let all_bishops = white_bishops | black_bishops;
                    if (all_bishops & WHITE_FIELDS) == all_bishops || (all_bishops & BLACK_FIELDS) == all_bishops {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Gets pieces count by counting set bits in occupancy.
    pub fn get_pieces_count(&self) -> u8 {
        bit_count(self.occupancy[WHITE as usize] | self.occupancy[BLACK as usize])
    }
}

impl BitboardState {
    /// Constructs a new instance of [BitboardState] with stored `halfmove_clock`, `castling_rights`, `en_passant`, `hash`, `pawn_hash` and `captured_piece`.
    pub fn new(halfmove_clock: u16, castling_rights: u8, en_passant: u64, hash: u64, pawn_hash: u64, captured_piece: u8) -> BitboardState {
        BitboardState { halfmove_clock, castling_rights, en_passant, hash, pawn_hash, captured_piece }
    }
}
