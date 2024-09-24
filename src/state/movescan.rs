use super::representation::Board;
use super::representation::CastlingRights;
use super::*;
use crate::engine;
use crate::evaluation::mobility::MobilityAuxData;
use crate::evaluation::mobility::PieceMobility;
use crate::utils::bitflags::BitFlags;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::panic_fast;
use crate::utils::rand;
use std::cmp;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::MaybeUninit;

#[allow(non_snake_case)]
pub mod MoveFlags {
    pub const SINGLE_PUSH: u8 = 0;
    pub const DOUBLE_PUSH: u8 = 1;
    pub const SHORT_CASTLING: u8 = 2;
    pub const LONG_CASTLING: u8 = 3;
    pub const CAPTURE: u8 = 4;
    pub const EN_PASSANT: u8 = 5;
    pub const UNDEFINED1: u8 = 6;
    pub const UNDEFINED2: u8 = 7;
    pub const KNIGHT_PROMOTION: u8 = 8;
    pub const BISHOP_PROMOTION: u8 = 9;
    pub const ROOK_PROMOTION: u8 = 10;
    pub const QUEEN_PROMOTION: u8 = 11;
    pub const KNIGHT_PROMOTION_CAPTURE: u8 = 12;
    pub const BISHOP_PROMOTION_CAPTURE: u8 = 13;
    pub const ROOK_PROMOTION_CAPTURE: u8 = 14;
    pub const QUEEN_PROMOTION_CAPTURE: u8 = 15;

    pub const BIT_SPECIAL_0: u8 = 1;
    pub const BIT_SPECIAL_1: u8 = 2;
    pub const BIT_CAPTURE: u8 = 4;
    pub const BIT_PROMOTION: u8 = 8;
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub data: u16,
}

impl Move {
    /// Constructs a new instance of [Move] with stored `from`, `to` and `flags`.
    pub fn new(from: usize, to: usize, flags: u8) -> Self {
        Self { data: ((flags as u16) << 12) | ((to as u16) << 6) | (from as u16) }
    }

    /// Constructs a new instance of [Move] using raw bits, which will be directly used as a data.
    pub const fn new_from_raw(data: u16) -> Self {
        Self { data }
    }

    /// Constructs a new instance of [Move] with random values, not restricted by chess rules.
    pub fn new_random() -> Self {
        let from = rand::usize(ALL_SQUARES);
        let to = rand::usize(ALL_SQUARES);
        let mut flags = MoveFlags::UNDEFINED1;

        loop {
            if flags == MoveFlags::UNDEFINED1 || flags == MoveFlags::UNDEFINED2 {
                flags = rand::u8(0..16);
            } else {
                break;
            }
        }

        Move::new(from, to, flags)
    }

    /// Gets source square from the internal data.
    pub fn get_from(&self) -> usize {
        (self.data & 0x3f) as usize
    }

    /// Gets destination square from the internal data.
    pub fn get_to(&self) -> usize {
        ((self.data >> 6) & 0x3f) as usize
    }

    /// Gets flags from the internal data.
    pub fn get_flags(&self) -> u8 {
        (self.data >> 12) as u8
    }

    /// Gets promotion piece based on the flags saved in the internal data.
    pub fn get_promotion_piece(&self) -> usize {
        let flags = self.get_flags();
        match self.get_flags() {
            MoveFlags::KNIGHT_PROMOTION | MoveFlags::KNIGHT_PROMOTION_CAPTURE => KNIGHT,
            MoveFlags::BISHOP_PROMOTION | MoveFlags::BISHOP_PROMOTION_CAPTURE => BISHOP,
            MoveFlags::ROOK_PROMOTION | MoveFlags::ROOK_PROMOTION_CAPTURE => ROOK,
            MoveFlags::QUEEN_PROMOTION | MoveFlags::QUEEN_PROMOTION_CAPTURE => QUEEN,
            _ => panic_fast!("Invalid value: flags={:?}", flags),
        }
    }

    pub fn is_some(&self) -> bool {
        self.data != 0
    }

    pub fn is_empty(&self) -> bool {
        self.data == 0
    }

    /// Checks if the move is a single push.
    pub fn is_single_push(&self) -> bool {
        self.get_flags() == MoveFlags::SINGLE_PUSH
    }

    /// Checks if the move is a double push.
    pub fn is_double_push(&self) -> bool {
        self.get_flags() == MoveFlags::DOUBLE_PUSH
    }

    /// Checks if the move is quiet (single or double pushes).
    pub fn is_quiet(&self) -> bool {
        self.get_flags() == MoveFlags::SINGLE_PUSH || self.get_flags() == MoveFlags::DOUBLE_PUSH
    }

    /// Checks if the move is a capture (excluding en passant, but including promotions).
    pub fn is_capture(&self) -> bool {
        self.get_flags().contains(MoveFlags::CAPTURE)
    }

    /// Checks if the move is en passant.
    pub fn is_en_passant(&self) -> bool {
        self.get_flags() == MoveFlags::EN_PASSANT
    }

    /// Checks if the move is a promotion (including captures).
    pub fn is_promotion(&self) -> bool {
        self.get_flags().contains(MoveFlags::BIT_PROMOTION)
    }

    /// Checks if the move is a short or long castling.
    pub fn is_castling(&self) -> bool {
        self.get_flags() == MoveFlags::SHORT_CASTLING || self.get_flags() == MoveFlags::LONG_CASTLING
    }

    /// Checks if the move is legal, using `board` as the context.
    pub fn is_legal(&self, board: &Board) -> bool {
        let from = self.get_from();
        let from_bb = 1u64 << from;
        let to = self.get_to();
        let to_bb = 1u64 << to;
        let piece = board.get_piece(from);
        let piece_color = board.get_piece_color(from);

        // Fast check: source square must contain a piece with the proper color
        if piece == usize::MAX || piece_color != board.active_color {
            return false;
        }

        let target_piece = board.get_piece(to);
        let target_piece_color = board.get_piece_color(to);

        // Fast check: for promotions with captures, there must be some victim with opposite color
        if self.is_capture() && !self.is_en_passant() && (target_piece == usize::MAX || piece_color == target_piece_color) {
            return false;
        }

        // Fast check: target square must be empty for non-capture moves
        if !self.is_capture() && target_piece != usize::MAX {
            return false;
        }

        let flags = self.get_flags();
        let occupancy_bb = board.occupancy[WHITE] | board.occupancy[BLACK];

        // Checking what squares are reachable for the piece
        let moves_bb = match piece {
            PAWN => match flags {
                MoveFlags::DOUBLE_PUSH => match board.active_color {
                    WHITE => from_bb << 16,
                    BLACK => from_bb >> 16,
                    _ => panic_fast!("Invalid value: board.active_color={}", board.active_color),
                },
                MoveFlags::EN_PASSANT => board.state.en_passant,
                _ => match board.active_color {
                    WHITE => ((from_bb & !FILE_H_BB) << 7) | ((from_bb & !RANK_8_BB) << 8) | ((from_bb & !FILE_A_BB) << 9),
                    BLACK => ((from_bb & !FILE_A_BB) >> 7) | ((from_bb & !RANK_1_BB) >> 8) | ((from_bb & !FILE_H_BB) >> 9),
                    _ => panic_fast!("Invalid value: board.active_color={}", board.active_color),
                },
            },
            KNIGHT => board.magic.get_knight_moves(from),
            BISHOP => board.magic.get_bishop_moves(occupancy_bb, from),
            ROOK => board.magic.get_rook_moves(occupancy_bb, from),
            QUEEN => board.magic.get_queen_moves(occupancy_bb, from),
            KING => match flags {
                MoveFlags::SHORT_CASTLING => 1u64 << (from - 2),
                MoveFlags::LONG_CASTLING => 1u64 << (from + 2),
                _ => board.magic.get_king_moves(from),
            },
            _ => panic_fast!("Invalid value: fen={}, piece={}", board, piece),
        };

        // Target square must be valid in this position
        if (moves_bb & to_bb) == 0 {
            return false;
        }

        // Special rules, not covered by fast checks
        if self.is_single_push() {
            if piece == PAWN {
                // Pawn at promotion rank, but without proper flags
                if (to_bb & (RANK_1_BB | RANK_8_BB)) != 0 {
                    return false;
                }

                // Non-capture pawn move can only go straight by one rank
                if ((from as i8) - (to as i8)).abs() != 8 {
                    return false;
                }
            }

            true
        } else if self.is_double_push() {
            // Double push can be performed only by pawns
            if piece != PAWN {
                return false;
            }

            // Double push can be performed only from the specific ranks (2 for white, 7 for black)
            if (board.active_color == WHITE && (from_bb & RANK_2_BB) == 0) || (board.active_color == BLACK && (from_bb & RANK_7_BB) == 0) {
                return false;
            }

            // The square between source and target ones must be empty
            if board.get_piece((cmp::max(from, to) + cmp::min(from, to)) / 2) != usize::MAX {
                return false;
            }

            true
        } else if self.is_en_passant() {
            // En passant can be performed only by pawns
            if piece != PAWN {
                return false;
            }

            true
        } else if self.is_promotion() {
            // Promotion can be performed only by pawns
            if piece != PAWN {
                return false;
            }

            if self.is_capture() {
                let direction = ((from as i8) - (to as i8)).abs();

                // Captures with pawns can go only diagonally
                if direction != 7 && direction != 9 {
                    return false;
                }
            }

            true
        } else if self.is_capture() {
            if piece == PAWN {
                let direction = ((from as i8) - (to as i8)).abs();

                // Captures with pawns can go only diagonally
                if direction != 7 && direction != 9 {
                    return false;
                }
            }

            true
        } else if self.is_castling() {
            // Castling can be performed only by kings
            if piece != KING {
                return false;
            }

            // Castling can be performed only from the specific squares (e1 for white, e8 for black)
            if (board.active_color == WHITE && from != E1) || (board.active_color == BLACK && from != E8) {
                return false;
            }

            let castling_right = match flags {
                MoveFlags::SHORT_CASTLING => match piece_color {
                    WHITE => CastlingRights::WHITE_SHORT_CASTLING,
                    BLACK => CastlingRights::BLACK_SHORT_CASTLING,
                    _ => panic_fast!("Invalid value: fen={}, piece_color={}", board, piece_color),
                },
                MoveFlags::LONG_CASTLING => match piece_color {
                    WHITE => CastlingRights::WHITE_LONG_CASTLING,
                    BLACK => CastlingRights::BLACK_LONG_CASTLING,
                    _ => panic_fast!("Invalid value: fen={}, piece_color={}", board, piece_color),
                },
                _ => panic_fast!("Invalid value: fen={}, flags={:?}", board, flags),
            };

            // There must be a proper castling right to perform it
            if !board.state.castling_rights.contains(castling_right) {
                return false;
            }

            let rook_square_bb = match flags {
                MoveFlags::SHORT_CASTLING => match piece_color {
                    WHITE => H1_BB,
                    BLACK => H8_BB,
                    _ => panic_fast!("Invalid value: fen={}, piece_color={}", board, piece_color),
                },
                MoveFlags::LONG_CASTLING => match piece_color {
                    WHITE => A1_BB,
                    BLACK => A8_BB,
                    _ => panic_fast!("Invalid value: fen={}, piece_color={}", board, piece_color),
                },
                _ => panic_fast!("Invalid value: fen={}, flags={:?}", board, flags),
            };

            // There must be a rook on the specific square
            if (board.pieces[board.active_color][ROOK] & rook_square_bb) == 0 {
                return false;
            }

            let castling_area_bb = match flags {
                MoveFlags::SHORT_CASTLING => match piece_color {
                    WHITE => F1_BB | G1_BB,
                    BLACK => F8_BB | G8_BB,
                    _ => panic_fast!("Invalid value: fen={}, piece_color={}", board, piece_color),
                },
                MoveFlags::LONG_CASTLING => match piece_color {
                    WHITE => B1_BB | C1_BB | D1_BB,
                    BLACK => B8_BB | C8_BB | D8_BB,
                    _ => panic_fast!("Invalid value: fen={}, piece_color={}", board, piece_color),
                },
                _ => panic_fast!("Invalid value: fen={}, flags={:?}", board, flags),
            };

            // There must be a free space for castling
            if (castling_area_bb & occupancy_bb) != 0 {
                return false;
            }

            true
        } else {
            panic_fast!("Move legality check failed: fen={}, self.data={}", board, self.data);
        }
    }
}

impl Default for Move {
    /// Constructs a new instance of [Move] with zeroed values.
    fn default() -> Self {
        Move::new(0, 0, MoveFlags::SINGLE_PUSH)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_long_notation())
    }
}

/// Generates all possible non-captures (if `CAPTURES` is false) or all possible captures (if `CAPTURES` is true) for the `PIECE` at
/// the position specified by `board`, stores them into `moves` list (starting from `index`) and returns index of the first free slot.
/// Use `evasion_mask` with value different than `u64::MAX` to restrict generator to the specified squares (useful during checks).
pub fn scan_piece_moves<const PIECE: usize, const CAPTURES: bool>(
    board: &Board,
    moves: &mut [MaybeUninit<Move>; engine::MAX_MOVES_COUNT],
    mut index: usize,
    evasion_mask: u64,
) -> usize {
    let enemy_color = board.active_color ^ 1;
    let mut pieces_bb = board.pieces[board.active_color][PIECE];

    while pieces_bb != 0 {
        let from_bb = pieces_bb.get_lsb();
        let from = from_bb.bit_scan();
        pieces_bb = pieces_bb.pop_lsb();

        let occupancy_bb = board.occupancy[WHITE] | board.occupancy[BLACK];
        let mut piece_moves_bb = match PIECE {
            KNIGHT => board.magic.get_knight_moves(from),
            BISHOP => board.magic.get_bishop_moves(occupancy_bb, from),
            ROOK => board.magic.get_rook_moves(occupancy_bb, from),
            QUEEN => board.magic.get_queen_moves(occupancy_bb, from),
            KING => board.magic.get_king_moves(from),
            _ => panic_fast!("Invalid parameter: fen={}, PIECE={}", board, PIECE),
        };
        piece_moves_bb &= !board.occupancy[board.active_color] & evasion_mask;

        if CAPTURES {
            piece_moves_bb &= board.occupancy[enemy_color];
        } else {
            piece_moves_bb &= !board.occupancy[enemy_color];
        }

        while piece_moves_bb != 0 {
            let to_bb = piece_moves_bb.get_lsb();
            let to = to_bb.bit_scan();
            piece_moves_bb = piece_moves_bb.pop_lsb();

            let capture = (to_bb & board.occupancy[enemy_color]) != 0;
            let flags = if CAPTURES || capture { MoveFlags::CAPTURE } else { MoveFlags::SINGLE_PUSH };

            moves[index].write(Move::new(from, to, flags));
            index += 1;
        }

        if PIECE == KING && !CAPTURES {
            match board.active_color {
                WHITE => {
                    let king_side_castling_rights = board.state.castling_rights.contains(CastlingRights::WHITE_SHORT_CASTLING);
                    if king_side_castling_rights && (occupancy_bb & (F1_BB | G1_BB)) == 0 {
                        if !board.are_squares_attacked(board.active_color, &[E1, F1, G1]) {
                            moves[index].write(Move::new(E1, G1, MoveFlags::SHORT_CASTLING));
                            index += 1;
                        }
                    }

                    let queen_side_castling_rights = board.state.castling_rights.contains(CastlingRights::WHITE_LONG_CASTLING);
                    if queen_side_castling_rights && (occupancy_bb & (B1_BB | C1_BB | D1_BB)) == 0 {
                        if !board.are_squares_attacked(board.active_color, &[C1, D1, E1]) {
                            moves[index].write(Move::new(E1, C1, MoveFlags::LONG_CASTLING));
                            index += 1;
                        }
                    }
                }
                BLACK => {
                    let king_side_castling_rights = board.state.castling_rights.contains(CastlingRights::BLACK_SHORT_CASTLING);
                    if king_side_castling_rights && (occupancy_bb & (F8_BB | G8_BB)) == 0 {
                        if !board.are_squares_attacked(board.active_color, &[E8, F8, G8]) {
                            moves[index].write(Move::new(E8, G8, MoveFlags::SHORT_CASTLING));
                            index += 1;
                        }
                    }

                    let queen_side_castling_rights = board.state.castling_rights.contains(CastlingRights::BLACK_LONG_CASTLING);
                    if queen_side_castling_rights && (occupancy_bb & (B8_BB | C8_BB | D8_BB)) == 0 {
                        if !board.are_squares_attacked(board.active_color, &[C8, D8, E8]) {
                            moves[index].write(Move::new(E8, C8, MoveFlags::LONG_CASTLING));
                            index += 1;
                        }
                    }
                }
                _ => panic_fast!("Invalid value: board.active_color={}", board.active_color),
            }
        }
    }

    index
}

/// Gets `PIECE` mobility (by counting all possible moves at the position specified by `board`) with `color` and increases `dangered_king_squares` if the enemy
/// king is near to the squares included in the mobility.
pub fn get_piece_mobility<const PIECE: usize>(board: &Board, color: usize, aux: &mut MobilityAuxData) -> PieceMobility {
    debug_assert!(color < 2);

    let mut pieces_bb = board.pieces[color][PIECE];
    let mut mobility_inner = 0;
    let mut mobility_outer = 0;

    let enemy_color = color ^ 1;
    let enemy_king_square = (board.pieces[enemy_color][KING]).bit_scan();
    let enemy_king_box_bb = patterns::get_box(enemy_king_square);

    while pieces_bb != 0 {
        let from_bb = pieces_bb.get_lsb();
        let from = from_bb.bit_scan();
        pieces_bb = pieces_bb.pop_lsb();

        let mut occupancy_bb = board.occupancy[WHITE] | board.occupancy[BLACK];
        occupancy_bb &= !match PIECE {
            BISHOP => board.pieces[color][BISHOP] | board.pieces[color][QUEEN],
            ROOK => board.pieces[color][ROOK] | board.pieces[color][QUEEN],
            QUEEN => board.pieces[color][BISHOP] | board.pieces[color][ROOK] | board.pieces[color][QUEEN],
            _ => 0,
        };

        let mut piece_moves_bb = match PIECE {
            KNIGHT => board.magic.get_knight_moves(from),
            BISHOP => board.magic.get_bishop_moves(occupancy_bb, from),
            ROOK => board.magic.get_rook_moves(occupancy_bb, from),
            QUEEN => board.magic.get_queen_moves(occupancy_bb, from),
            KING => board.magic.get_king_moves(from),
            _ => panic_fast!("Invalid parameter: fen={}, PIECE={}", board, PIECE),
        };

        aux.king_area_threats += (enemy_king_box_bb & (piece_moves_bb | from_bb)).bit_count() as i8;
        piece_moves_bb &= !board.occupancy[color] & !board.pawn_attacks[enemy_color];

        mobility_inner += (piece_moves_bb & CENTER_BB).bit_count() as i8;
        mobility_outer += (piece_moves_bb & OUTSIDE_BB).bit_count() as i8;
    }

    PieceMobility { inner: mobility_inner, outer: mobility_outer }
}

/// Generates all possible non-captures (if `CAPTURES` is false) or all possible captures (if `CAPTURES` is true) for the pawns at
/// the position specified by `board`, stores them into `moves` list (starting from `index`) and returns index of the first free slot.
/// Use `evasion_mask` with value different than `u64::MAX` to restrict generator to the specified squares (useful during checks).
pub fn scan_pawn_moves<const CAPTURES: bool>(
    board: &Board,
    moves: &mut [MaybeUninit<Move>; engine::MAX_MOVES_COUNT],
    mut index: usize,
    evasion_mask: u64,
) -> usize {
    if !CAPTURES {
        index = scan_pawn_moves_single_push(board, moves, index, evasion_mask);
        index = scan_pawn_moves_double_push(board, moves, index, evasion_mask);
    } else {
        index = scan_pawn_moves_diagonal_attacks::<LEFT>(board, moves, index, evasion_mask);
        index = scan_pawn_moves_diagonal_attacks::<RIGHT>(board, moves, index, evasion_mask);
    }

    index
}

/// Generates all possible single pushes for the pawns at the position specified by `board`, stores them into `moves` list (starting from `index`)
/// and returns index of the first free slot. Use `evasion_mask` with value different than `u64::MAX` to restrict generator to the
/// specified squares (useful during checks).
fn scan_pawn_moves_single_push(board: &Board, moves: &mut [MaybeUninit<Move>; engine::MAX_MOVES_COUNT], mut index: usize, evasion_mask: u64) -> usize {
    let pieces_bb = board.pieces[board.active_color][PAWN];
    let occupancy_bb = board.occupancy[WHITE] | board.occupancy[BLACK];

    let shift = 8 - 16 * (board.active_color as i8);
    let promotion_rank_bb = RANK_8_BB >> (56 * (board.active_color as u8));
    let mut target_squares_bb = match board.active_color {
        WHITE => pieces_bb << 8,
        BLACK => pieces_bb >> 8,
        _ => {
            panic_fast!("Invalid value: board.active_color={}", board.active_color);
        }
    };
    target_squares_bb &= !occupancy_bb & evasion_mask;

    while target_squares_bb != 0 {
        let to_bb = target_squares_bb.get_lsb();
        let to = to_bb.bit_scan();
        let from = ((to as i8) - shift) as usize;
        target_squares_bb = target_squares_bb.pop_lsb();

        if (to_bb & promotion_rank_bb) != 0 {
            moves[index + 0].write(Move::new(from, to, MoveFlags::QUEEN_PROMOTION));
            moves[index + 1].write(Move::new(from, to, MoveFlags::ROOK_PROMOTION));
            moves[index + 2].write(Move::new(from, to, MoveFlags::BISHOP_PROMOTION));
            moves[index + 3].write(Move::new(from, to, MoveFlags::KNIGHT_PROMOTION));
            index += 4;
        } else {
            moves[index].write(Move::new(from, to, MoveFlags::SINGLE_PUSH));
            index += 1;
        }
    }

    index
}

/// Generates all possible double pushes for the pawns at the position specified by `board`, stores them into `moves` list (starting from `index`)
/// and returns index of the first free slot. Use `evasion_mask` with value different than `u64::MAX` to restrict generator to the
/// specified squares (useful during checks).
fn scan_pawn_moves_double_push(board: &Board, moves: &mut [MaybeUninit<Move>; engine::MAX_MOVES_COUNT], mut index: usize, evasion_mask: u64) -> usize {
    let pieces_bb = board.pieces[board.active_color][PAWN];
    let occupancy_bb = board.occupancy[WHITE] | board.occupancy[BLACK];

    let shift = 16 - 32 * (board.active_color as i8);
    let mut target_squares_bb = match board.active_color {
        WHITE => (((pieces_bb & RANK_2_BB) << 8) & !occupancy_bb) << 8,
        BLACK => (((pieces_bb & RANK_7_BB) >> 8) & !occupancy_bb) >> 8,
        _ => {
            panic_fast!("Invalid value: board.active_color={}", board.active_color);
        }
    };
    target_squares_bb &= !occupancy_bb & evasion_mask;

    while target_squares_bb != 0 {
        let to_bb = target_squares_bb.get_lsb();
        let to = to_bb.bit_scan();
        let from = ((to as i8) - shift) as usize;
        target_squares_bb = target_squares_bb.pop_lsb();

        moves[index].write(Move::new(from, to, MoveFlags::DOUBLE_PUSH));
        index += 1;
    }

    index
}

/// Generates all possible captures for the pawns toward the direction specified by `DIR` and at the position specified by `board`,
/// stores them into `moves` list (starting from `index`) and returns index of the first free slot. Use `evasion_mask` with value
/// different than `u64::MAX` to restrict generator to the specified squares (useful during checks).
fn scan_pawn_moves_diagonal_attacks<const DIR: usize>(
    board: &Board,
    moves: &mut [MaybeUninit<Move>; engine::MAX_MOVES_COUNT],
    mut index: usize,
    evasion_mask: u64,
) -> usize {
    let enemy_color = board.active_color ^ 1;
    let pieces_bb = board.pieces[board.active_color][PAWN];

    let forbidden_file_bb = FILE_A_BB >> (DIR * 7);
    let shift = 9 - (board.active_color ^ DIR) * 2;
    let signed_shift = (shift as i8) - ((board.active_color as i8) * 2 * (shift as i8));
    let promotion_rank_bb = RANK_8_BB >> (56 * (board.active_color as u8));

    let mut target_squares_bb = match board.active_color {
        WHITE => (pieces_bb & !forbidden_file_bb) << shift,
        BLACK => (pieces_bb & !forbidden_file_bb) >> shift,
        _ => {
            panic_fast!("Invalid value: board.active_color={}", board.active_color);
        }
    };
    target_squares_bb &= (board.occupancy[enemy_color] | board.state.en_passant) & evasion_mask;

    while target_squares_bb != 0 {
        let to_bb = target_squares_bb.get_lsb();
        let to = to_bb.bit_scan();
        let from = ((to as i8) - signed_shift) as usize;
        target_squares_bb = target_squares_bb.pop_lsb();

        if (to_bb & promotion_rank_bb) != 0 {
            moves[index + 0].write(Move::new(from, to, MoveFlags::QUEEN_PROMOTION_CAPTURE));
            moves[index + 1].write(Move::new(from, to, MoveFlags::ROOK_PROMOTION_CAPTURE));
            moves[index + 2].write(Move::new(from, to, MoveFlags::BISHOP_PROMOTION_CAPTURE));
            moves[index + 3].write(Move::new(from, to, MoveFlags::KNIGHT_PROMOTION_CAPTURE));
            index += 4;
        } else {
            let en_passant = (to_bb & board.state.en_passant) != 0;
            let flags = if en_passant { MoveFlags::EN_PASSANT } else { MoveFlags::CAPTURE };

            moves[index].write(Move::new(from, to, flags));
            index += 1;
        }
    }

    index
}
