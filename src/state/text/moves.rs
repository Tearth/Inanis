use crate::engine;
use crate::state::movescan::Move;
use crate::state::movescan::MoveFlags;
use crate::state::representation::Board;
use crate::state::*;
use crate::utils::bitflags::BitFlags;
use std::mem::MaybeUninit;

impl Move {
    /// Converts short-notated move (e4, Rc8, Qxb6) in `text` into the [Move] instance, using the `board` as context.
    /// Returns [Err] with the proper message if `text` couldn't be parsed correctly.
    pub fn from_short_notation(mut text: &str, board: &mut Board) -> Result<Move, String> {
        let mut moves = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
        let moves_count = board.get_all_moves(&mut moves, u64::MAX);

        let mut desired_to: Option<usize> = None;
        let mut desired_file: Option<usize> = None;
        let mut desired_rank: Option<usize> = None;
        let mut desired_piece: Option<usize> = None;
        let mut desired_flags: Option<u8> = None;
        let mut desired_capture: Option<bool> = None;
        let mut desired_promotion: Option<usize> = None;

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

                    desired_to = Some(to as usize);
                    desired_piece = Some(PAWN);
                }
                // Nd5
                3 => {
                    let piece = chars.next().ok_or(format!("Invalid move, bad piece: text={}", text))?;
                    let file = chars.next().ok_or(format!("Invalid move, bad destination file: text={}", text))? as u8;
                    let rank = chars.next().ok_or(format!("Invalid move, bad destination rank: text={}", text))? as u8;

                    let to = (7 - (file - b'a')) + 8 * (rank - b'1');
                    let piece_type = text::symbol_to_piece(piece)?;

                    desired_to = Some(to as usize);
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

                            desired_to = Some(to as usize);
                            desired_file = Some(file_from as usize);
                            desired_piece = Some(PAWN);
                            desired_capture = Some(true);
                        // Rxf5
                        } else {
                            let piece_type = text::symbol_to_piece(piece_or_file)?;

                            desired_to = Some(to as usize);
                            desired_piece = Some(piece_type);
                            desired_capture = Some(true);
                        }
                    // N3e4
                    } else if capture_or_file_rank.is_ascii_digit() {
                        let piece_type = text::symbol_to_piece(piece_or_file)?;
                        let rank_from = (capture_or_file_rank as u8) - b'1';

                        desired_to = Some(to as usize);
                        desired_piece = Some(piece_type);
                        desired_rank = Some(rank_from as usize);
                    }
                    // Nde4
                    else {
                        let file_from = 7 - ((capture_or_file_rank as u8) - b'a');
                        let piece_type = text::symbol_to_piece(piece_or_file)?;

                        desired_to = Some(to as usize);
                        desired_piece = Some(piece_type);
                        desired_file = Some(file_from as usize);
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
                    let piece_type = text::symbol_to_piece(piece)?;

                    if capture_or_rank == 'x' {
                        // R2xc2
                        if file_or_rank.is_ascii_digit() {
                            let rank_from = (file_or_rank as u8) - b'1';

                            desired_to = Some(to as usize);
                            desired_rank = Some(rank_from as usize);
                            desired_piece = Some(piece_type);
                            desired_capture = Some(true);
                        // Rexc2
                        } else {
                            let file_from = 7 - ((file_or_rank as u8) - b'a');

                            desired_to = Some(to as usize);
                            desired_file = Some(file_from as usize);
                            desired_piece = Some(piece_type);
                            desired_capture = Some(true);
                        }
                    // Qd3c2
                    } else {
                        let file_from = 7 - ((file_or_rank as u8) - b'a');
                        let rank_from = (capture_or_rank as u8) - b'1';

                        desired_to = Some(to as usize);
                        desired_file = Some(file_from as usize);
                        desired_rank = Some(rank_from as usize);
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
                    let piece_type = text::symbol_to_piece(piece)?;

                    let file_from = 7 - ((source_file as u8) - b'a');
                    let rank_from = (source_rank as u8) - b'1';

                    desired_to = Some(to as usize);
                    desired_file = Some(file_from as usize);
                    desired_rank = Some(rank_from as usize);
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
    pub fn from_long_notation(text: &str, board: &Board) -> Result<Move, String> {
        let mut chars = text.chars();
        let from_file = chars.next().ok_or(format!("Invalid move, bad source file: text={}", text))? as u8;
        let from_rank = chars.next().ok_or(format!("Invalid move, bad source rank: text={}", text))? as u8;
        let to_file = chars.next().ok_or(format!("Invalid move, bad destination file: text={}", text))? as u8;
        let to_rank = chars.next().ok_or(format!("Invalid move, bad destination rank: text={}", text))? as u8;
        let promotion = chars.next();

        if !(b'a'..=b'h').contains(&from_file) || !(b'a'..=b'h').contains(&to_file) {
            return Err(format!("Invalid move, bad source square: fen={}, text={}", board.to_fen(), text));
        }

        if !(b'1'..=b'8').contains(&from_rank) || !(b'1'..=b'8').contains(&to_rank) {
            return Err(format!("Invalid move, bad destination square: fen={}, text={}", board.to_fen(), text));
        }

        if let Some(promotion_piece) = promotion {
            if !['n', 'b', 'r', 'q'].contains(&promotion_piece) {
                return Err(format!("Invalid move, bad promotion piece: fen={}, text={}", board.to_fen(), text));
            }
        }

        let from = ((7 - (from_file - b'a')) + 8 * (from_rank - b'1')) as usize;
        let to = ((7 - (to_file - b'a')) + 8 * (to_rank - b'1')) as usize;
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
            None => MoveFlags::SINGLE_PUSH,
        };

        let mut moves = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
        let moves_count = board.get_all_moves(&mut moves, u64::MAX);

        for r#move in &moves[0..moves_count] {
            let r#move = unsafe { r#move.assume_init() };
            if r#move.get_from() == from && r#move.get_to() == to {
                let flags = r#move.get_flags();
                if promotion_flags == MoveFlags::SINGLE_PUSH || (flags & promotion_flags) == flags {
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
            char::from(b'a' + (7 - from % 8) as u8),
            char::from(b'1' + (from / 8) as u8),
            char::from(b'a' + (7 - to % 8) as u8),
            char::from(b'1' + (to / 8) as u8),
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
}
