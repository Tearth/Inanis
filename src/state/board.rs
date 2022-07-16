use super::movegen::MagicContainer;
use super::movescan;
use super::movescan::Move;
use super::movescan::MoveFlags;
use super::patterns::PatternsContainer;
use super::zobrist::ZobristContainer;
use super::*;
use crate::cache::pawns::PawnHashTable;
use crate::engine::context::SearchStatistics;
use crate::engine::see::SEEContainer;
use crate::evaluation::material;
use crate::evaluation::mobility;
use crate::evaluation::pawns;
use crate::evaluation::pst;
use crate::evaluation::safety;
use crate::evaluation::EvaluationParameters;
use crate::utils::fen;
use std::mem::MaybeUninit;
use std::sync::Arc;

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
    pub captured_piece: u8,
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
    pub castling_rights: CastlingRights,
    pub en_passant: u64,
    pub hash: u64,
    pub pawn_hash: u64,
    pub captured_piece: u8,
}

impl Bitboard {
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

        Bitboard {
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
        Bitboard::new_from_fen(
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
    ) -> Result<Self, &'static str> {
        fen::fen_to_board(
            fen,
            evaluation_parameters,
            zobrist_container,
            patterns_container,
            see_container,
            magic_container,
        )
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
    ) -> Result<Self, &'static str> {
        let mut board = Bitboard::new_initial_position(evaluation_parameters, zobrist_container, patterns_container, see_container, magic_container);
        for premade_move in moves {
            let parsed_move = Move::from_long_notation(premade_move, &board)?;
            board.make_move(parsed_move);
        }

        Ok(board)
    }

    /// Generates all possible non-captures (if `CAPTURES` is false) or all possible captures (if `CAPTURES` is true) at the current position, stores
    /// them into `moves` list (starting from `index`) and returns index of the first free slot. Use `evasion_mask` with value different
    /// than `u64::MAX` to restrict generator to the specified fields (useful during checks).
    pub fn get_moves<const CAPTURES: bool>(&self, moves: &mut [MaybeUninit<Move>], mut index: usize, evasion_mask: u64) -> usize {
        index = movescan::scan_pawn_moves::<CAPTURES>(self, moves, index, evasion_mask);
        index = movescan::scan_piece_moves::<KNIGHT, CAPTURES>(self, moves, index, evasion_mask);
        index = movescan::scan_piece_moves::<BISHOP, CAPTURES>(self, moves, index, evasion_mask);
        index = movescan::scan_piece_moves::<ROOK, CAPTURES>(self, moves, index, evasion_mask);
        index = movescan::scan_piece_moves::<QUEEN, CAPTURES>(self, moves, index, evasion_mask);
        index = movescan::scan_piece_moves::<KING, CAPTURES>(self, moves, index, evasion_mask);

        index
    }

    /// Generates all possible moves (non-captures and captures) at the current position, stores them into `moves` list (starting from `index`) and returns
    /// index of the first free slot. Use `evasion_mask` with value different than `u64::MAX` to restrict generator to the specified fields (useful during checks).
    pub fn get_all_moves(&self, moves: &mut [MaybeUninit<Move>], evasion_mask: u64) -> usize {
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
            MoveFlags::QUIET => {
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
                let king_from = 3 + 56 * (color as u8);
                let king_to = 1 + 56 * (color as u8);

                self.move_piece(color, KING, king_from, king_to);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_from);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_to);

                let rook_from = 0 + 56 * (color as u8);
                let rook_to = 2 + 56 * (color as u8);

                self.move_piece(color, ROOK, rook_from, rook_to);
                self.hash ^= self.zobrist.get_piece_hash(color, ROOK, rook_from);
                self.hash ^= self.zobrist.get_piece_hash(color, ROOK, rook_to);
            }
            MoveFlags::LONG_CASTLING => {
                let king_from = 3 + 56 * (color as u8);
                let king_to = 5 + 56 * (color as u8);

                self.move_piece(color, KING, king_from, king_to);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_from);
                self.hash ^= self.zobrist.get_piece_hash(color, KING, king_to);

                let rook_from = 7 + 56 * (color as u8);
                let rook_to = 4 + 56 * (color as u8);

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

                let enemy_pawn_field_index = ((to as i8) + 8 * ((color as i8) * 2 - 1)) as u8;

                self.remove_piece(enemy_color, PAWN, enemy_pawn_field_index);
                self.hash ^= self.zobrist.get_piece_hash(enemy_color, piece, enemy_pawn_field_index);
                self.pawn_hash ^= self.zobrist.get_piece_hash(enemy_color, piece, enemy_pawn_field_index);
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
                _ => panic!("Invalid value: color={}", color),
            };

            self.pawn_hash ^= self.zobrist.get_piece_hash(color, KING, from);
            self.pawn_hash ^= self.zobrist.get_piece_hash(color, KING, to);
        }

        if piece == ROOK {
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

        self.hash ^= self.zobrist.get_active_color_hash();
        self.active_color = enemy_color;
    }

    /// Undoes `r#move`, with the assumption that it's perfectly valid at the current position (otherwise, internal state can be irreversibly corrupted).
    ///
    /// Steps of undoing a move:
    ///  - restore halfmove clock, castling rights, en passant bitboard, board hash and pawn hash
    ///  - update piece bitboards
    ///  - decrease fullmove number if needed
    ///  - restore halfmove clock if needed
    ///  - switch active color
    pub fn undo_move(&mut self, r#move: Move) {
        let color = self.active_color ^ 1;
        let enemy_color = self.active_color;
        let from = r#move.get_from();
        let to = r#move.get_to();
        let flags = r#move.get_flags();
        let piece = self.get_piece(to);

        match flags {
            MoveFlags::QUIET => {
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
    ///  - restore halfmove clock, castling rights, en passant bitboard, board hash and pawn hash
    ///  - decrease fullmove number if needed
    ///  - switch active color
    ///  - decrease null moves count
    pub fn undo_null_move(&mut self) {
        if self.active_color == WHITE {
            self.fullmove_number -= 1;
        }

        self.active_color ^= 1;
        self.null_moves -= 1;
        self.pop_state();
    }

    pub fn push_state(&mut self) {
        self.state_stack.push(BitboardState::new(
            self.halfmove_clock,
            self.castling_rights,
            self.en_passant,
            self.hash,
            self.pawn_hash,
            self.captured_piece,
        ));
    }

    pub fn pop_state(&mut self) {
        let state = unsafe { self.state_stack.pop().unwrap_unchecked() };
        self.halfmove_clock = state.halfmove_clock;
        self.castling_rights = state.castling_rights;
        self.en_passant = state.en_passant;
        self.hash = state.hash;
        self.pawn_hash = state.pawn_hash;
        self.captured_piece = state.captured_piece;
    }

    /// Checks if the field specified by `field_index` is attacked by enemy, from the `color` perspective.
    pub fn is_field_attacked(&self, color: u8, field_index: u8) -> bool {
        let enemy_color = color ^ 1;
        let occupancy = self.occupancy[WHITE as usize] | self.occupancy[BLACK as usize];

        let rook_queen_attacks = self.magic.get_rook_moves(occupancy, field_index as usize);
        let enemy_rooks_queens = self.pieces[enemy_color as usize][ROOK as usize] | self.pieces[enemy_color as usize][QUEEN as usize];
        if (rook_queen_attacks & enemy_rooks_queens) != 0 {
            return true;
        }

        let bishop_queen_attacks = self.magic.get_bishop_moves(occupancy, field_index as usize);
        let enemy_bishops_queens = self.pieces[enemy_color as usize][BISHOP as usize] | self.pieces[enemy_color as usize][QUEEN as usize];
        if (bishop_queen_attacks & enemy_bishops_queens) != 0 {
            return true;
        }

        let knight_attacks = self.magic.get_knight_moves(field_index as usize, &self.patterns);
        let enemy_knights = self.pieces[enemy_color as usize][KNIGHT as usize];
        if (knight_attacks & enemy_knights) != 0 {
            return true;
        }

        let king_attacks = self.magic.get_king_moves(field_index as usize, &self.patterns);
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

    /// Checks if any of the field specified by `field_indexes` list is attacked by enemy, from the `color` perspective.
    pub fn are_fields_attacked(&self, color: u8, field_indexes: &[u8]) -> bool {
        field_indexes.iter().any(|field_index| self.is_field_attacked(color, *field_index))
    }

    /// Gets a list of enemy pieces attacking a field specified by `fields_index`, from the `color` perspective. The encoding looks as follows:
    ///  - bit 0 - Pawn
    ///  - bit 1, 2, 3 - Knight/Bishop
    ///  - bit 4, 5 - Rook
    ///  - bit 6 - Queen
    ///  - bit 7 - King
    pub fn get_attacking_pieces(&self, color: u8, field_index: u8) -> u8 {
        let mut result = 0;
        let enemy_color = color ^ 1;
        let occupancy = self.occupancy[WHITE as usize] | self.occupancy[BLACK as usize];

        let bishops_rooks = self.pieces[enemy_color as usize][BISHOP as usize] | self.pieces[enemy_color as usize][ROOK as usize];
        let rooks_queens = self.pieces[enemy_color as usize][ROOK as usize] | self.pieces[enemy_color as usize][QUEEN as usize];
        let bishops_queens = self.pieces[enemy_color as usize][BISHOP as usize] | self.pieces[enemy_color as usize][QUEEN as usize];

        let king_attacks = self.magic.get_king_moves(field_index as usize, &self.patterns);
        if (king_attacks & self.pieces[enemy_color as usize][KING as usize]) != 0 {
            result |= 1 << 7;
        }

        let queen_attacks = self.magic.get_queen_moves(occupancy & !bishops_rooks, field_index as usize);
        if (queen_attacks & self.pieces[enemy_color as usize][QUEEN as usize]) != 0 {
            result |= 1 << 6;
        }

        let rook_attacks = self.magic.get_rook_moves(occupancy & !rooks_queens, field_index as usize);
        let attacking_rooks = rook_attacks & self.pieces[enemy_color as usize][ROOK as usize];
        if attacking_rooks != 0 {
            result |= match bit_count(attacking_rooks) {
                1 => 1 << 4,
                _ => 3 << 4,
            };
        }

        let mut attacking_knights_bishops_count = 0;

        let knight_attacks = self.magic.get_knight_moves(field_index as usize, &self.patterns);
        let enemy_knights = self.pieces[enemy_color as usize][KNIGHT as usize];
        let attacking_knights = knight_attacks & enemy_knights;
        if (knight_attacks & enemy_knights) != 0 {
            attacking_knights_bishops_count += bit_count(attacking_knights);
        }

        let bishop_attacks = self.magic.get_bishop_moves(occupancy & !bishops_queens, field_index as usize);
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

    /// Check if the king of the `color` side is checked.
    pub fn is_king_checked(&self, color: u8) -> bool {
        if self.pieces[color as usize][KING as usize] == 0 {
            return false;
        }

        self.is_field_attacked(color, bit_scan(self.pieces[color as usize][KING as usize]))
    }

    /// Gets piece on the field specified by `field_index`.
    pub fn get_piece(&self, field_index: u8) -> u8 {
        self.piece_table[field_index as usize]
    }

    /// Gets piece's color on the field specified by `field_index`. Returns `u8::MAX` if there is no piece there.
    pub fn get_piece_color(&self, field_index: u8) -> u8 {
        let piece = self.piece_table[field_index as usize];
        if piece == u8::MAX {
            return u8::MAX;
        }

        if ((1u64 << field_index) & self.occupancy[WHITE as usize]) != 0 {
            WHITE
        } else {
            BLACK
        }
    }

    /// Adds `piece` on the `field` with the specified `color`, also updates occupancy and incremental values.
    pub fn add_piece(&mut self, color: u8, piece: u8, field: u8) {
        self.pieces[color as usize][piece as usize] |= 1u64 << field;
        self.occupancy[color as usize] |= 1u64 << field;
        self.piece_table[field as usize] = piece;
        self.material_scores[color as usize] += self.evaluation_parameters.piece_value[piece as usize];

        self.pst_scores[color as usize][OPENING as usize] += self.evaluation_parameters.get_pst_value(color, piece, OPENING, field);
        self.pst_scores[color as usize][ENDING as usize] += self.evaluation_parameters.get_pst_value(color, piece, ENDING, field);
    }

    /// Removes `piece` on the `field` with the specified `color`, also updates occupancy and incremental values.
    pub fn remove_piece(&mut self, color: u8, piece: u8, field: u8) {
        self.pieces[color as usize][piece as usize] &= !(1u64 << field);
        self.occupancy[color as usize] &= !(1u64 << field);
        self.piece_table[field as usize] = u8::MAX;
        self.material_scores[color as usize] -= self.evaluation_parameters.piece_value[piece as usize];

        self.pst_scores[color as usize][OPENING as usize] -= self.evaluation_parameters.get_pst_value(color, piece, OPENING, field);
        self.pst_scores[color as usize][ENDING as usize] -= self.evaluation_parameters.get_pst_value(color, piece, ENDING, field);
    }

    /// Moves `piece` from the field specified by `from` to the field specified by `to` with the specified `color`, also updates occupancy and incremental values.
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
                    let field = get_lsb(pieces);
                    let field_index = bit_scan(field);
                    pieces = pop_lsb(pieces);

                    hash ^= self.zobrist.get_piece_hash(color, piece_index, field_index);
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
                    let field = get_lsb(pieces);
                    let field_index = bit_scan(field);
                    pieces = pop_lsb(pieces);

                    hash ^= self.zobrist.get_piece_hash(color, piece, field_index);
                }
            }
        }

        self.pawn_hash = hash;
    }

    /// Runs full evaluation (material, piece-square tables, mobility, pawns structure and safety) of the current position, using `pawn_hashtable` to store pawn
    /// evaluations and `statistics` to gather diagnostic data. Returns score from the white color perspective (more than 0 when advantage, less than 0 when disadvantage).
    pub fn evaluate<const DIAG: bool>(&self, pawn_hashtable: &PawnHashTable, statistics: &mut SearchStatistics) -> i16 {
        let mut white_attack_mask = 0;
        let mut black_attack_mask = 0;
        let mobility_score = mobility::evaluate(self, &mut white_attack_mask, &mut black_attack_mask);

        material::evaluate(self)
            + pst::evaluate(self)
            + pawns::evaluate::<DIAG>(self, pawn_hashtable, statistics)
            + safety::evaluate(self, white_attack_mask, black_attack_mask)
            + mobility_score
    }

    /// Runs full evaluation (material, piece-square tables, mobility, pawns structure and safety) of the current position.
    /// Returns score from the white color perspective (more than 0 when advantage, less than 0 when disadvantage).
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

    /// Runs lazy (fast) evaluations, considering only material and piece-square tables. Returns score from the white color perspective (more than 0 when
    /// advantage, less than 0 when disadvantage).
    pub fn evaluate_lazy(&self) -> i16 {
        material::evaluate(self) + pst::evaluate(self)
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

    /// Calculates a game phase at the current position: 1.0 means opening (all pieces present, considering the default position), 0.0 is ending (no pieces at all).
    pub fn get_game_phase(&self) -> f32 {
        let total_material = self.material_scores[WHITE as usize] + self.material_scores[BLACK as usize];
        let total_material_without_kings = total_material - 2 * self.evaluation_parameters.piece_value[KING as usize];

        (total_material_without_kings as f32) / (self.evaluation_parameters.initial_material as f32)
    }

    pub fn get_pieces_count(&self) -> u8 {
        bit_count(self.occupancy[WHITE as usize] | self.occupancy[BLACK as usize])
    }
}

impl BitboardState {
    pub fn new(halfmove_clock: u16, castling_rights: CastlingRights, en_passant: u64, hash: u64, pawn_hash: u64, captured_piece: u8) -> BitboardState {
        BitboardState {
            halfmove_clock,
            castling_rights,
            en_passant,
            hash,
            pawn_hash,
            captured_piece,
        }
    }
}
