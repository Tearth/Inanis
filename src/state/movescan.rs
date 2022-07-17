use super::board::Bitboard;
use super::board::CastlingRights;
use super::*;
use crate::engine;
use std::cmp;
use std::mem::MaybeUninit;

bitflags! {
    pub struct MoveFlags: u8 {
        const QUIET = 0;
        const DOUBLE_PUSH = 1;
        const SHORT_CASTLING = 2;
        const LONG_CASTLING = 3;
        const CAPTURE = 4;
        const EN_PASSANT = 5;
        const UNDEFINED1 = 6;
        const UNDEFINED2 = 7;
        const KNIGHT_PROMOTION = 8;
        const BISHOP_PROMOTION = 9;
        const ROOK_PROMOTION = 10;
        const QUEEN_PROMOTION = 11;
        const KNIGHT_PROMOTION_CAPTURE = 12;
        const BISHOP_PROMOTION_CAPTURE = 13;
        const ROOK_PROMOTION_CAPTURE = 14;
        const QUEEN_PROMOTION_CAPTURE = 15;

        const FIELD_SPECIAL_0 = 1;
        const FIELD_SPECIAL_1 = 2;
        const FIELD_CAPTURE = 4;
        const FIELD_PROMOTION = 8;
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub data: u16,
}

impl Move {
    /// Constructs a new instance of [Move] with stored `from`, `to` and `flags`.
    pub fn new(from: u8, to: u8, flags: MoveFlags) -> Self {
        Self {
            data: ((flags.bits as u16) << 12) | ((to as u16) << 6) | (from as u16),
        }
    }

    /// Constructs a new instance of [Move] using raw bits, which will be directly used as a data.
    pub fn new_from_raw(data: u16) -> Self {
        Self { data }
    }

    /// Constructs a new instance of [Move] with random values, not restricted by chess rules.
    pub fn new_random() -> Self {
        let from = fastrand::u8(0..64);
        let to = fastrand::u8(0..64);
        let mut flags = MoveFlags::UNDEFINED1;

        loop {
            if flags == MoveFlags::UNDEFINED1 || flags == MoveFlags::UNDEFINED2 {
                flags = MoveFlags::from_bits(fastrand::u8(0..16)).unwrap();
            } else {
                break;
            }
        }

        Move::new(from, to, flags)
    }

    /// Converts short-notated move (e4, Rc8, Qxb6) in `text` into the [Move] instance, using the `board` as context.
    /// Returns [Err] with the proper message if `text` couldn't be parsed correctly.
    pub fn from_short_notation(mut text: &str, board: &mut Bitboard) -> Result<Move, String> {
        let mut moves: [MaybeUninit<Move>; engine::MAX_MOVES_COUNT] = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
        let moves_count = board.get_all_moves(&mut moves, u64::MAX);

        let mut desired_to: Option<u8> = None;
        let mut desired_file: Option<u8> = None;
        let mut desired_rank: Option<u8> = None;
        let mut desired_piece: Option<u8> = None;
        let mut desired_flags: Option<MoveFlags> = None;
        let mut desired_capture: Option<bool> = None;
        let mut desired_promotion: Option<u8> = None;

        let original_text = text;
        text = text.trim_matches('#');
        text = text.trim_matches('+');
        text = text.trim_matches('=');
        text = text.trim_matches('?');
        text = text.trim_matches('!');

        if text.contains('=') {
            let promotion = &text[text.len() - 2..];
            desired_promotion = match promotion {
                "=Q" => Some(QUEEN),
                "=R" => Some(ROOK),
                "=B" => Some(BISHOP),
                "=N" => Some(KNIGHT),
                _ => return Err(format!("Invalid promotion: fen={}, promotion={}", board.to_fen(), promotion)),
            }
        }

        text = text.trim_end_matches("=Q");
        text = text.trim_end_matches("=R");
        text = text.trim_end_matches("=B");
        text = text.trim_end_matches("=N");

        if text == "0-0" || text == "O-O" {
            desired_piece = Some(KING);
            desired_flags = Some(MoveFlags::SHORT_CASTLING);
        } else if text == "0-0-0" || text == "O-O-O" {
            desired_piece = Some(KING);
            desired_flags = Some(MoveFlags::LONG_CASTLING);
        } else {
            let mut chars = text.chars();
            match text.len() {
                // e4
                2 => {
                    let file = chars.next().ok_or(format!("Invalid move, bad destination file: text={}", text))? as u8;
                    let rank = chars.next().ok_or(format!("Invalid move, bad source rank: text={}", text))? as u8;
                    let to = (7 - (file - b'a')) + 8 * (rank - b'1');

                    desired_to = Some(to);
                    desired_piece = Some(PAWN);
                }
                // Nd5
                3 => {
                    let piece = chars.next().ok_or(format!("Invalid move, bad piece: text={}", text))?;
                    let file = chars.next().ok_or(format!("Invalid move, bad destination file: text={}", text))? as u8;
                    let rank = chars.next().ok_or(format!("Invalid move, bad destination rank: text={}", text))? as u8;

                    let to = (7 - (file - b'a')) + 8 * (rank - b'1');
                    let piece_type = symbol_to_piece(piece)?;

                    desired_to = Some(to);
                    desired_piece = Some(piece_type);
                }
                // exf5, Rxf5, N3e4, Nde4
                4 => {
                    let piece_or_file = chars.next().ok_or(format!("Invalid move, bad symbol: text={}", text))?;
                    let capture_or_file_rank = chars.next().ok_or(format!("Invalid move, bad symbol: text={}", text))?;
                    let file = chars.next().ok_or(format!("Invalid move, bad destination file: text={}", text))? as u8;
                    let rank = chars.next().ok_or(format!("Invalid move, bad destination rank: text={}", text))? as u8;
                    let to = (7 - (file - b'a')) + 8 * (rank - b'1');

                    // exf5, Rxf5
                    if capture_or_file_rank == 'x' {
                        // exf5
                        if piece_or_file.is_lowercase() {
                            let file_from = 7 - ((piece_or_file as u8) - b'a');

                            desired_to = Some(to);
                            desired_file = Some(file_from);
                            desired_piece = Some(PAWN);
                            desired_capture = Some(true);
                        // Rxf5
                        } else {
                            let piece_type = symbol_to_piece(piece_or_file)?;

                            desired_to = Some(to);
                            desired_piece = Some(piece_type);
                            desired_capture = Some(true);
                        }
                    // N3e4
                    } else if capture_or_file_rank.is_ascii_digit() {
                        let piece_type = symbol_to_piece(piece_or_file)?;
                        let rank_from = (capture_or_file_rank as u8) - b'1';

                        desired_to = Some(to);
                        desired_piece = Some(piece_type);
                        desired_rank = Some(rank_from);
                    }
                    // Nde4
                    else {
                        let file_from = 7 - ((capture_or_file_rank as u8) - b'a');
                        let piece_type = symbol_to_piece(piece_or_file)?;

                        desired_to = Some(to);
                        desired_piece = Some(piece_type);
                        desired_file = Some(file_from);
                    }
                }
                // R2xc2, Rexc2, Qd3c2
                5 => {
                    let piece = chars.next().ok_or(format!("Invalid move, bad piece: text={}", text))?;
                    let file_or_rank = chars.next().ok_or(format!("Invalid move, bad symbol: text={}", text))?;
                    let capture_or_rank = chars.next().ok_or(format!("Invalid move, bad symbol: text={}", text))?;
                    let file = chars.next().ok_or(format!("Invalid move, bad destination file: text={}", text))? as u8;
                    let rank = chars.next().ok_or(format!("Invalid move, bad destination rank: text={}", text))? as u8;
                    let to = (7 - (file - b'a')) + 8 * (rank - b'1');
                    let piece_type = symbol_to_piece(piece)?;

                    if capture_or_rank == 'x' {
                        // R2xc2
                        if file_or_rank.is_ascii_digit() {
                            let rank_from = (file_or_rank as u8) - b'1';

                            desired_to = Some(to);
                            desired_rank = Some(rank_from);
                            desired_piece = Some(piece_type);
                            desired_capture = Some(true);
                        // Rexc2
                        } else {
                            let file_from = 7 - ((file_or_rank as u8) - b'a');

                            desired_to = Some(to);
                            desired_file = Some(file_from);
                            desired_piece = Some(piece_type);
                            desired_capture = Some(true);
                        }
                    // Qd3c2
                    } else {
                        let file_from = 7 - ((file_or_rank as u8) - b'a');
                        let rank_from = (capture_or_rank as u8) - b'1';

                        desired_to = Some(to);
                        desired_file = Some(file_from);
                        desired_rank = Some(rank_from);
                        desired_piece = Some(piece_type);
                    }
                }
                // Qa1xd4
                6 => {
                    let piece = chars.next().ok_or(format!("Invalid move, bad piece: text={}", text))?;
                    let source_file = chars.next().ok_or(format!("Invalid move, bad source file: text={}", text))?;
                    let source_rank = chars.next().ok_or(format!("Invalid move, bad source rank: text={}", text))?;
                    let _ = chars.next().ok_or(format!("Invalid move, bad symbol: text={}", text))?;
                    let file = chars.next().ok_or(format!("Invalid move, bad destination file: text={}", text))? as u8;
                    let rank = chars.next().ok_or(format!("Invalid move, bad destination rank: text={}", text))? as u8;
                    let to = (7 - (file - b'a')) + 8 * (rank - b'1');
                    let piece_type = symbol_to_piece(piece)?;

                    let file_from = 7 - ((source_file as u8) - b'a');
                    let rank_from = (source_rank as u8) - b'1';

                    desired_to = Some(to);
                    desired_file = Some(file_from);
                    desired_rank = Some(rank_from);
                    desired_piece = Some(piece_type);
                    desired_capture = Some(true);
                }
                _ => return Err(format!("Invalid move: fen={}, original_text={}", board.to_fen(), original_text)),
            }
        }

        let mut valid_moves = Vec::new();
        for move_index in 0..moves_count {
            let r#move = unsafe { moves[move_index].assume_init() };
            if (desired_to.is_none() || desired_to.unwrap() == r#move.get_to())
                && (desired_file.is_none() || (r#move.get_from() % 8) == desired_file.unwrap())
                && (desired_rank.is_none() || (r#move.get_from() / 8) == desired_rank.unwrap())
                && (desired_piece.is_none() || board.get_piece(r#move.get_from()) == desired_piece.unwrap())
                && (desired_flags.is_none() || r#move.get_flags() == desired_flags.unwrap())
                && (desired_capture.is_none() || r#move.is_capture() == desired_capture.unwrap())
                && (desired_promotion.is_none() || (r#move.is_promotion() && r#move.get_promotion_piece() == desired_promotion.unwrap()))
            {
                valid_moves.push(r#move);
            }
        }

        match valid_moves.len() {
            0 => Err(format!("Invalid move: fen={}, original_text={}", board.to_fen(), original_text)),
            1 => Ok(valid_moves[0]),
            _ => {
                for r#move in valid_moves {
                    board.make_move(r#move);
                    if !board.is_king_checked(board.active_color ^ 1) {
                        board.undo_move(r#move);
                        return Ok(r#move);
                    }
                    board.undo_move(r#move);
                }

                Err(format!("Invalid move: fen={}, original_text={}", board.to_fen(), original_text))
            }
        }
    }

    /// Converts long-notated move (e2e4, a1a8) in `text` into the [Move] instance, using the `board` as context.
    /// Returns [Err] with the proper message if `text` couldn't be parsed correctly.
    pub fn from_long_notation(text: &str, board: &Bitboard) -> Result<Move, String> {
        let mut chars = text.chars();
        let from_file = chars.next().ok_or(format!("Invalid move, bad source file: text={}", text))? as u8;
        let from_rank = chars.next().ok_or(format!("Invalid move, bad source rank: text={}", text))? as u8;
        let to_file = chars.next().ok_or(format!("Invalid move, bad destination file: text={}", text))? as u8;
        let to_rank = chars.next().ok_or(format!("Invalid move, bad destination rank: text={}", text))? as u8;
        let promotion = chars.next();

        if !(b'a'..=b'h').contains(&from_file) || !(b'a'..=b'h').contains(&to_file) {
            return Err(format!("Invalid move, bad source field: fen={}, text={}", board.to_fen(), text));
        }

        if !(b'1'..=b'8').contains(&from_rank) || !(b'1'..=b'8').contains(&to_rank) {
            return Err(format!("Invalid move, bad destination field: fen={}, text={}", board.to_fen(), text));
        }

        if let Some(promotion_piece) = promotion {
            if !['n', 'b', 'r', 'q'].contains(&promotion_piece) {
                return Err(format!("Invalid move, bad promotion piece: fen={}, text={}", board.to_fen(), text));
            }
        }

        let from = (7 - (from_file - b'a')) + 8 * (from_rank - b'1');
        let to = (7 - (to_file - b'a')) + 8 * (to_rank - b'1');
        let promotion_flags = match promotion {
            Some(promotion_piece) => {
                let mut flags = match promotion_piece {
                    'n' => MoveFlags::KNIGHT_PROMOTION,
                    'b' => MoveFlags::BISHOP_PROMOTION,
                    'r' => MoveFlags::ROOK_PROMOTION,
                    'q' => MoveFlags::QUEEN_PROMOTION,
                    _ => return Err(format!("Invalid move, bad promotion piece: fen={}, text={}", board.to_fen(), text)),
                };

                if ((from as i8) - (to as i8)).abs() != 8 {
                    flags |= MoveFlags::CAPTURE;
                };

                flags
            }
            None => MoveFlags::QUIET,
        };

        let mut moves: [MaybeUninit<Move>; engine::MAX_MOVES_COUNT] = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
        let moves_count = board.get_all_moves(&mut moves, u64::MAX);

        for r#move in &moves[0..moves_count] {
            let r#move = unsafe { r#move.assume_init() };
            if r#move.get_from() == from && r#move.get_to() == to {
                let flags = r#move.get_flags();
                if promotion_flags == MoveFlags::QUIET || (flags & promotion_flags).bits == flags.bits {
                    return Ok(r#move);
                }
            }
        }

        Err(format!("Invalid move: fen={}, text={}", board.to_fen(), text))
    }

    /// Converts move into the long notation (e2e4, a1a8).
    pub fn to_long_notation(self) -> String {
        let from = self.get_from();
        let to = self.get_to();

        let mut result = vec![
            char::from(b'a' + (7 - from % 8)),
            char::from(b'1' + from / 8),
            char::from(b'a' + (7 - to % 8)),
            char::from(b'1' + to / 8),
        ];

        let flags = self.get_flags();
        if flags.contains(MoveFlags::KNIGHT_PROMOTION) {
            result.push(match flags {
                MoveFlags::KNIGHT_PROMOTION | MoveFlags::KNIGHT_PROMOTION_CAPTURE => 'n',
                MoveFlags::BISHOP_PROMOTION | MoveFlags::BISHOP_PROMOTION_CAPTURE => 'b',
                MoveFlags::ROOK_PROMOTION | MoveFlags::ROOK_PROMOTION_CAPTURE => 'r',
                MoveFlags::QUEEN_PROMOTION | MoveFlags::QUEEN_PROMOTION_CAPTURE => 'q',
                _ => panic!("Invalid value: flags={:?}", flags),
            });
        }

        result.into_iter().collect()
    }

    /// Gets source field from the internal data.
    pub fn get_from(&self) -> u8 {
        (self.data & 0x3f) as u8
    }

    /// Gets destination field from the internal data.
    pub fn get_to(&self) -> u8 {
        ((self.data >> 6) & 0x3f) as u8
    }

    /// Gets flags from the internal data.
    pub fn get_flags(&self) -> MoveFlags {
        unsafe { MoveFlags::from_bits_unchecked((self.data >> 12) as u8) }
    }

    /// Gets promotion piece based on the flags saved in the internal data.
    pub fn get_promotion_piece(&self) -> u8 {
        match self.get_flags() {
            MoveFlags::KNIGHT_PROMOTION | MoveFlags::KNIGHT_PROMOTION_CAPTURE => KNIGHT,
            MoveFlags::BISHOP_PROMOTION | MoveFlags::BISHOP_PROMOTION_CAPTURE => BISHOP,
            MoveFlags::ROOK_PROMOTION | MoveFlags::ROOK_PROMOTION_CAPTURE => ROOK,
            MoveFlags::QUEEN_PROMOTION | MoveFlags::QUEEN_PROMOTION_CAPTURE => QUEEN,
            _ => panic!("Invalid value: self.get_flags()={:?}", self.get_flags()),
        }
    }

    /// Checks if the move is quiet (single or double pushes).
    pub fn is_quiet(&self) -> bool {
        self.get_flags() == MoveFlags::QUIET || self.get_flags() == MoveFlags::DOUBLE_PUSH
    }

    /// Checks if the move is a double push.
    pub fn is_double_push(&self) -> bool {
        self.get_flags() == MoveFlags::DOUBLE_PUSH
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
        self.get_flags().contains(MoveFlags::FIELD_PROMOTION)
    }

    /// Checks if the move is a short or long castling.
    pub fn is_castling(&self) -> bool {
        self.get_flags() == MoveFlags::SHORT_CASTLING || self.get_flags() == MoveFlags::LONG_CASTLING
    }

    /// Checks if the move is legal, using `board` as the context.
    pub fn is_legal(&self, board: &Bitboard) -> bool {
        let from = self.get_from();
        let from_field = 1u64 << from;
        let to = self.get_to();
        let to_field = 1u64 << to;
        let piece = board.get_piece(from);
        let piece_color = board.get_piece_color(from);

        if piece == u8::MAX || piece_color != board.active_color {
            return false;
        }

        if (self.is_double_push() || self.is_en_passant() || self.is_promotion()) && piece != PAWN {
            return false;
        }

        if self.is_castling() && piece != KING {
            return false;
        }

        let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
        let moves = match piece {
            PAWN => match self.get_flags() {
                MoveFlags::DOUBLE_PUSH => {
                    if (board.active_color == WHITE && (from_field & 65280) == 0) || (board.active_color == BLACK && (from_field & 71776119061217280) == 0) {
                        return false;
                    }

                    match board.active_color {
                        WHITE => 1u64.checked_shl(from.wrapping_add(16) as u32).unwrap_or(0),
                        BLACK => 1u64.checked_shl(from.wrapping_sub(16) as u32).unwrap_or(0),
                        _ => panic!("Invalid value: board.active_color={}", board.active_color),
                    }
                }
                MoveFlags::EN_PASSANT => board.en_passant,
                _ => {
                    (match board.active_color {
                        WHITE => {
                            1u64.checked_shl((from.wrapping_add(7)) as u32).unwrap_or(0)
                                | 1u64.checked_shl((from.wrapping_add(8)) as u32).unwrap_or(0)
                                | 1u64.checked_shl((from.wrapping_add(9)) as u32).unwrap_or(0)
                        }
                        BLACK => {
                            1u64.checked_shl((from.wrapping_sub(7)) as u32).unwrap_or(0)
                                | 1u64.checked_shl((from.wrapping_sub(8)) as u32).unwrap_or(0)
                                | 1u64.checked_shl((from.wrapping_sub(9)) as u32).unwrap_or(0)
                        }
                        _ => panic!("Invalid value: board.active_color={}", board.active_color),
                    }) & board.patterns.get_box(from as usize)
                }
            },
            KNIGHT => board.magic.get_knight_moves(from as usize, &board.patterns),
            BISHOP => board.magic.get_bishop_moves(occupancy, from as usize),
            ROOK => board.magic.get_rook_moves(occupancy, from as usize),
            QUEEN => board.magic.get_queen_moves(occupancy, from as usize),
            KING => {
                if self.is_castling() {
                    if (board.active_color == WHITE && from != 3) || (board.active_color == BLACK && from != 59) {
                        return false;
                    }
                }

                match self.get_flags() {
                    MoveFlags::SHORT_CASTLING => 1u64 << (from - 2),
                    MoveFlags::LONG_CASTLING => 1u64 << (from + 2),
                    _ => board.magic.get_king_moves(from as usize, &board.patterns),
                }
            }
            _ => panic!("Invalid value: fen={}, piece={}", board.to_fen(), piece),
        };

        if (moves & to_field) == 0 {
            return false;
        }

        let target_piece = board.get_piece(to);
        let target_piece_color = board.get_piece_color(to);

        if self.is_quiet() {
            if self.is_double_push() {
                let middle_field_index = (cmp::max(from, to) + cmp::min(from, to)) / 2;
                if board.get_piece(middle_field_index) != u8::MAX {
                    return false;
                }
            }

            if target_piece == u8::MAX {
                return true;
            }
        } else if self.is_en_passant() {
            if piece == PAWN && target_piece == u8::MAX {
                return true;
            }
        } else if self.is_promotion() {
            if piece == PAWN {
                if !self.is_capture() && target_piece == u8::MAX {
                    return true;
                } else if self.is_capture() && target_piece != u8::MAX && target_piece != KING && piece_color != target_piece_color {
                    return true;
                }
            }
        } else if self.is_capture() {
            if target_piece != u8::MAX && target_piece != KING && piece_color != target_piece_color {
                return true;
            }
        } else if self.is_castling() {
            if piece == KING && target_piece == u8::MAX {
                let mut rook_field = 0;
                let mut rook_target_field = 0;

                if (board.active_color == WHITE && from != 3) || (board.active_color == BLACK && from != 59) {
                    return false;
                }

                match self.get_flags() {
                    MoveFlags::SHORT_CASTLING => {
                        rook_field = from - 3;
                        rook_target_field = from - 1;
                    }
                    MoveFlags::LONG_CASTLING => {
                        rook_field = from + 4;
                        rook_target_field = from + 1;
                    }
                    _ => panic!("Invalid value: fen={}, self.get_flags()={:?}", board.to_fen(), self.get_flags()),
                };

                if board.get_piece(rook_target_field) == u8::MAX
                    && board.get_piece(rook_field) == ROOK
                    && board.get_piece_color(rook_field) == board.active_color
                {
                    return true;
                }
            }
        } else {
            panic!("Move legality check failed: fen={}, self.data={}", board.to_fen(), self.data);
        }

        false
    }
}

impl Default for Move {
    /// Constructs a new instance of [Move] with zeroed values.
    fn default() -> Self {
        Move::new(0, 0, MoveFlags::QUIET)
    }
}

/// Generates all possible non-captures (if `CAPTURES` is false) or all possible captures (if `CAPTURES` is true) for the `PIECE` at
/// the position specified by `board`, stores them into `moves` list (starting from `index`) and returns index of the first free slot.
/// Use `evasion_mask` with value different than `u64::MAX` to restrict generator to the specified fields (useful during checks).
pub fn scan_piece_moves<const PIECE: u8, const CAPTURES: bool>(
    board: &Bitboard,
    moves: &mut [MaybeUninit<Move>],
    mut index: usize,
    evasion_mask: u64,
) -> usize {
    let enemy_color = board.active_color ^ 1;
    let mut pieces = board.pieces[board.active_color as usize][PIECE as usize];

    while pieces != 0 {
        let from_field = get_lsb(pieces);
        let from_field_index = bit_scan(from_field);
        pieces = pop_lsb(pieces);

        let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
        let mut piece_moves = match PIECE {
            KNIGHT => board.magic.get_knight_moves(from_field_index as usize, &board.patterns),
            BISHOP => board.magic.get_bishop_moves(occupancy, from_field_index as usize),
            ROOK => board.magic.get_rook_moves(occupancy, from_field_index as usize),
            QUEEN => board.magic.get_queen_moves(occupancy, from_field_index as usize),
            KING => board.magic.get_king_moves(from_field_index as usize, &board.patterns),
            _ => panic!("Invalid parameter: fen={}, PIECE={}", board.to_fen(), PIECE),
        };
        piece_moves &= !board.occupancy[board.active_color as usize] & evasion_mask;

        if CAPTURES {
            piece_moves &= board.occupancy[enemy_color as usize];
        } else {
            piece_moves &= !board.occupancy[enemy_color as usize];
        }

        while piece_moves != 0 {
            let to_field = get_lsb(piece_moves);
            let to_field_index = bit_scan(to_field);
            piece_moves = pop_lsb(piece_moves);

            let capture = (to_field & board.occupancy[enemy_color as usize]) != 0;
            let flags = if CAPTURES || capture { MoveFlags::CAPTURE } else { MoveFlags::QUIET };

            moves[index].write(Move::new(from_field_index, to_field_index, flags));
            index += 1;
        }

        if PIECE == KING && !CAPTURES {
            match board.active_color {
                WHITE => {
                    let king_side_castling_rights = board.castling_rights.contains(CastlingRights::WHITE_SHORT_CASTLING);
                    let king_side_rook_present = (board.pieces[board.active_color as usize][ROOK as usize] & 0x1) != 0;

                    if king_side_castling_rights && king_side_rook_present && (occupancy & 0x6) == 0 {
                        if !board.are_fields_attacked(board.active_color, &[3, 2, 1]) {
                            moves[index].write(Move::new(3, 1, MoveFlags::SHORT_CASTLING));
                            index += 1;
                        }
                    }

                    let queen_side_castling_rights = board.castling_rights.contains(CastlingRights::WHITE_LONG_CASTLING);
                    let queen_side_rook_present = (board.pieces[board.active_color as usize][ROOK as usize] & 0x80) != 0;

                    if queen_side_castling_rights && queen_side_rook_present && (occupancy & 0x70) == 0 {
                        if !board.are_fields_attacked(board.active_color, &[3, 4, 5]) {
                            moves[index].write(Move::new(3, 5, MoveFlags::LONG_CASTLING));
                            index += 1;
                        }
                    }
                }
                BLACK => {
                    let king_side_castling_rights = board.castling_rights.contains(CastlingRights::BLACK_SHORT_CASTLING);
                    let king_side_rook_present = (board.pieces[board.active_color as usize][ROOK as usize] & 0x100000000000000) != 0;

                    if king_side_castling_rights && king_side_rook_present && (occupancy & 0x600000000000000) == 0 {
                        if !board.are_fields_attacked(board.active_color, &[59, 58, 57]) {
                            moves[index].write(Move::new(59, 57, MoveFlags::SHORT_CASTLING));
                            index += 1;
                        }
                    }

                    let queen_side_castling_rights = board.castling_rights.contains(CastlingRights::BLACK_LONG_CASTLING);
                    let queen_side_rook_present = (board.pieces[board.active_color as usize][ROOK as usize] & 0x8000000000000000) != 0;

                    if queen_side_castling_rights && queen_side_rook_present && (occupancy & 0x7000000000000000) == 0 {
                        if !board.are_fields_attacked(board.active_color, &[59, 60, 61]) {
                            moves[index].write(Move::new(59, 61, MoveFlags::LONG_CASTLING));
                            index += 1;
                        }
                    }
                }
                _ => panic!("Invalid value: board.active_color={}", board.active_color),
            }
        }
    }

    index
}

/// Gets `PIECE` mobility (by counting all possible moves at the position specified by `board`) with `color` and increases `dangered_king_fields` if the enemy
/// king is near to the fields included in the mobility.
pub fn get_piece_mobility<const PIECE: u8>(board: &Bitboard, color: u8, dangered_king_fields: &mut u32) -> i16 {
    let mut pieces = board.pieces[color as usize][PIECE as usize];
    let mut mobility = 0;

    let enemy_color = color ^ 1;
    let enemy_king_field = bit_scan(board.pieces[enemy_color as usize][KING as usize]);
    let enemy_king_box = board.patterns.get_box(enemy_king_field as usize);

    while pieces != 0 {
        let from_field = get_lsb(pieces);
        let from_field_index = bit_scan(from_field);
        pieces = pop_lsb(pieces);

        let mut occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
        occupancy &= !match PIECE {
            BISHOP => board.pieces[color as usize][BISHOP as usize] | board.pieces[color as usize][QUEEN as usize],
            ROOK => board.pieces[color as usize][ROOK as usize] | board.pieces[color as usize][QUEEN as usize],
            QUEEN => board.pieces[color as usize][BISHOP as usize] | board.pieces[color as usize][ROOK as usize] | board.pieces[color as usize][QUEEN as usize],
            _ => 0,
        };

        let piece_moves = match PIECE {
            KNIGHT => board.magic.get_knight_moves(from_field_index as usize, &board.patterns),
            BISHOP => board.magic.get_bishop_moves(occupancy, from_field_index as usize),
            ROOK => board.magic.get_rook_moves(occupancy, from_field_index as usize),
            QUEEN => board.magic.get_queen_moves(occupancy, from_field_index as usize),
            KING => board.magic.get_king_moves(from_field_index as usize, &board.patterns),
            _ => panic!("Invalid parameter: fen={}, PIECE={}", board.to_fen(), PIECE),
        } & !board.occupancy[color as usize];

        let center_mobility = board.evaluation_parameters.mobility_center_multiplier[PIECE as usize] * bit_count(piece_moves & CENTER) as i16;
        let outside_mobility = bit_count(piece_moves & OUTSIDE) as i16;

        mobility += center_mobility + outside_mobility;
        *dangered_king_fields += bit_count(enemy_king_box & piece_moves) as u32;
    }

    mobility
}

/// Generates all possible non-captures (if `CAPTURES` is false) or all possible captures (if `CAPTURES` is true) for the pawns at
/// the position specified by `board`, stores them into `moves` list (starting from `index`) and returns index of the first free slot.
/// Use `evasion_mask` with value different than `u64::MAX` to restrict generator to the specified fields (useful during checks).
pub fn scan_pawn_moves<const CAPTURES: bool>(board: &Bitboard, moves: &mut [MaybeUninit<Move>], mut index: usize, evasion_mask: u64) -> usize {
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
/// specified fields (useful during checks).
fn scan_pawn_moves_single_push(board: &Bitboard, moves: &mut [MaybeUninit<Move>], mut index: usize, evasion_mask: u64) -> usize {
    let pieces = board.pieces[board.active_color as usize][PAWN as usize];
    let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];

    let shift = 8 - 16 * (board.active_color as i8);
    let promotion_line = RANK_H >> (56 * (board.active_color as u8));
    let mut target_fields = match board.active_color {
        WHITE => (pieces << 8),
        BLACK => (pieces >> 8),
        _ => {
            panic!("Invalid value: board.active_color={}", board.active_color);
        }
    };
    target_fields &= !occupancy & evasion_mask;

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        let from_field_index = ((to_field_index as i8) - shift) as u8;
        target_fields = pop_lsb(target_fields);

        if (to_field & promotion_line) != 0 {
            moves[index + 0].write(Move::new(from_field_index, to_field_index, MoveFlags::QUEEN_PROMOTION));
            moves[index + 1].write(Move::new(from_field_index, to_field_index, MoveFlags::ROOK_PROMOTION));
            moves[index + 2].write(Move::new(from_field_index, to_field_index, MoveFlags::BISHOP_PROMOTION));
            moves[index + 3].write(Move::new(from_field_index, to_field_index, MoveFlags::KNIGHT_PROMOTION));
            index += 4;
        } else {
            moves[index].write(Move::new(from_field_index, to_field_index, MoveFlags::QUIET));
            index += 1;
        }
    }

    index
}

/// Generates all possible double pushes for the pawns at the position specified by `board`, stores them into `moves` list (starting from `index`)
/// and returns index of the first free slot. Use `evasion_mask` with value different than `u64::MAX` to restrict generator to the
/// specified fields (useful during checks).
fn scan_pawn_moves_double_push(board: &Bitboard, moves: &mut [MaybeUninit<Move>], mut index: usize, evasion_mask: u64) -> usize {
    let pieces = board.pieces[board.active_color as usize][PAWN as usize];
    let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];

    let shift = 16 - 32 * (board.active_color as i8);
    let mut target_fields = match board.active_color {
        WHITE => ((((pieces & RANK_B) << 8) & !occupancy) << 8),
        BLACK => ((((pieces & RANK_G) >> 8) & !occupancy) >> 8),
        _ => {
            panic!("Invalid value: board.active_color={}", board.active_color);
        }
    };
    target_fields &= !occupancy & evasion_mask;

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        let from_field_index = ((to_field_index as i8) - shift) as u8;
        target_fields = pop_lsb(target_fields);

        moves[index].write(Move::new(from_field_index, to_field_index, MoveFlags::DOUBLE_PUSH));
        index += 1;
    }

    index
}

/// Generates all possible captures for the pawns toward the direction specified by `DIR` and at the position specified by `board`,
/// stores them into `moves` list (starting from `index`) and returns index of the first free slot. Use `evasion_mask` with value
/// different than `u64::MAX` to restrict generator to the specified fields (useful during checks).
fn scan_pawn_moves_diagonal_attacks<const DIR: u8>(board: &Bitboard, moves: &mut [MaybeUninit<Move>], mut index: usize, evasion_mask: u64) -> usize {
    let enemy_color = board.active_color ^ 1;
    let pieces = board.pieces[board.active_color as usize][PAWN as usize];

    let forbidden_file = FILE_A >> (DIR * 7);
    let shift = 9 - (board.active_color ^ DIR) * 2;
    let signed_shift = (shift as i8) - ((board.active_color as i8) * 2 * (shift as i8));
    let promotion_line = RANK_H >> (56 * (board.active_color as u8));

    let mut target_fields = match board.active_color {
        WHITE => ((pieces & !forbidden_file) << shift),
        BLACK => ((pieces & !forbidden_file) >> shift),
        _ => {
            panic!("Invalid value: board.active_color={}", board.active_color);
        }
    };
    target_fields &= (board.occupancy[enemy_color as usize] | board.en_passant) & evasion_mask;

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        let from_field_index = ((to_field_index as i8) - signed_shift) as u8;
        target_fields = pop_lsb(target_fields);

        if (to_field & promotion_line) != 0 {
            moves[index + 0].write(Move::new(from_field_index, to_field_index, MoveFlags::QUEEN_PROMOTION_CAPTURE));
            moves[index + 1].write(Move::new(from_field_index, to_field_index, MoveFlags::ROOK_PROMOTION_CAPTURE));
            moves[index + 2].write(Move::new(from_field_index, to_field_index, MoveFlags::BISHOP_PROMOTION_CAPTURE));
            moves[index + 3].write(Move::new(from_field_index, to_field_index, MoveFlags::KNIGHT_PROMOTION_CAPTURE));
            index += 4;
        } else {
            let en_passant = (to_field & board.en_passant) != 0;
            let flags = if en_passant { MoveFlags::EN_PASSANT } else { MoveFlags::CAPTURE };

            moves[index].write(Move::new(from_field_index, to_field_index, flags));
            index += 1;
        }
    }

    index
}
