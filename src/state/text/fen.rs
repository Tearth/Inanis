use crate::engine::see::SEEContainer;
use crate::evaluation::EvaluationParameters;
use crate::state::movegen::MagicContainer;
use crate::state::patterns::PatternsContainer;
use crate::state::representation::Board;
use crate::state::representation::CastlingRights;
use crate::state::zobrist::ZobristContainer;
use crate::state::*;
use crate::utils::bitflags::BitFlags;
use crate::utils::bithelpers::BitHelpers;
use std::sync::Arc;

pub struct ParsedEPD {
    pub board: Board,
    pub id: Option<String>,
    pub best_move: Option<String>,
    pub comment: Option<String>,
}

impl ParsedEPD {
    /// Constructs a new instance of [ParsedEPD] with the `board` and rest of the squares zeroed.
    pub fn new(board: Board) -> Self {
        Self { board, id: None, best_move: None, comment: None }
    }
}

/// Converts `fen` into the [Board], using provided containers. If the parameter is [None], then the new container is created.
/// Returns [Err] with proper error message if `fen` couldn't be parsed correctly.
pub fn fen_to_board(
    fen: &str,
    evaluation_parameters: Option<Arc<EvaluationParameters>>,
    zobrist_container: Option<Arc<ZobristContainer>>,
    patterns_container: Option<Arc<PatternsContainer>>,
    see_container: Option<Arc<SEEContainer>>,
    magic_container: Option<Arc<MagicContainer>>,
) -> Result<Board, String> {
    Ok(epd_to_board(fen, evaluation_parameters, zobrist_container, patterns_container, see_container, magic_container)?.board)
}

/// Converts `epd` into the [Board], using provided containers. If the parameter is [None], then the new container is created.
/// Returns [Err] with proper error message if `epd` couldn't be parsed correctly.
pub fn epd_to_board(
    epd: &str,
    evaluation_parameters: Option<Arc<EvaluationParameters>>,
    zobrist_container: Option<Arc<ZobristContainer>>,
    patterns_container: Option<Arc<PatternsContainer>>,
    see_container: Option<Arc<SEEContainer>>,
    magic_container: Option<Arc<MagicContainer>>,
) -> Result<ParsedEPD, String> {
    let tokens: Vec<&str> = epd.split(' ').map(|v| v.trim()).collect();
    if tokens.len() < 4 {
        return Err(format!("Invalid FEN, input too short: epd={}", epd));
    }

    let mut board = Board::new(evaluation_parameters, zobrist_container, patterns_container, see_container, magic_container);
    fen_to_pieces(&mut board, tokens[0])?;
    fen_to_active_color(&mut board, tokens[1])?;
    fen_to_castling(&mut board, tokens[2])?;
    fen_to_en_passant(&mut board, tokens[3])?;

    board.recalculate_hashes();
    board.recalculate_incremental_values();

    if tokens.len() > 4 {
        let halfmove_clock_result = fen_to_halfmove_clock(&mut board, tokens[4]);
        let fullmove_number_result = fen_to_fullmove_number(&mut board, tokens[5]);

        // We are in EPD mode if halfmove clock and fullmove number are not present
        if halfmove_clock_result.is_err() && fullmove_number_result.is_err() {
            let mut parsed_epd = ParsedEPD::new(board);
            parsed_epd.id = get_epd_parameter(epd, &["id"]);
            parsed_epd.best_move = get_epd_parameter(epd, &["bm"]);
            parsed_epd.comment = get_epd_parameter(epd, &["c0", "c9"]);

            return Ok(parsed_epd);
        }
    }

    Ok(ParsedEPD::new(board))
}

/// Converts [Board] into the FEN.
pub fn board_to_fen(board: &Board) -> String {
    let pieces = pieces_to_fen(board);
    let active_color = active_color_to_fen(board);
    let castling = castling_to_fen(board);
    let en_passant = en_passant_to_fen(board);
    let halfmove_clock = halfmove_clock_to_fen(board);
    let fullmove_number = fullmove_number_to_fen(board);

    format!("{} {} {} {} {} {}", pieces, active_color, castling, en_passant, halfmove_clock, fullmove_number)
}

/// Converts [Board] into the EPD.
pub fn board_to_epd(board: &Board) -> String {
    let pieces = pieces_to_fen(board);
    let active_color = active_color_to_fen(board);
    let castling = castling_to_fen(board);
    let en_passant = en_passant_to_fen(board);

    format!("{} {} {} {}", pieces, active_color, castling, en_passant)
}

/// Gets a value of the `name` parameters from the specified `epd`. Returns [None] if the parameter was not found.
fn get_epd_parameter(mut epd: &str, name: &[&str]) -> Option<String> {
    let parameter_index = name.iter().find_map(|p| epd.find(p));
    if parameter_index.is_none() {
        return None;
    }

    epd = &epd[parameter_index.unwrap()..];

    let value_index = epd.find(' ');
    if value_index.is_none() {
        return None;
    }

    epd = &epd[value_index.unwrap()..];

    let separator_index = epd.find(';');
    if separator_index.is_none() {
        return None;
    }

    let mut value = &epd[0..separator_index.unwrap()];
    value = value.trim_matches(' ');
    value = value.trim_matches('\"');

    Some(value.to_string())
}

/// Parses FEN's pieces and stores them into the `board`. Returns [Err] with the proper error message if `pieces` couldn't be parsed.
fn fen_to_pieces(board: &mut Board, pieces: &str) -> Result<(), String> {
    let mut current_square = 63;
    for char in pieces.chars().filter(|&x| x != '/') {
        if char.is_ascii_digit() {
            current_square -= char.to_digit(10).ok_or(format!("Invalid FEN, bad symbol: pieces={}", pieces))? as i32;
        } else {
            let color = if char.is_uppercase() { WHITE } else { BLACK };
            let piece = text::symbol_to_piece(char)?;

            board.add_piece::<false>(color, piece, current_square as usize);
            current_square -= 1;
        }
    }

    Ok(())
}

/// Converts pieces from the `board` into the FEN chunk.
fn pieces_to_fen(board: &Board) -> String {
    let mut result = String::new();
    let mut squares_without_piece = 0;

    for square in (ALL_SQUARES).rev() {
        let piece = board.get_piece(square);
        if piece == usize::MAX {
            squares_without_piece += 1;
        } else {
            if squares_without_piece != 0 {
                result.push(char::from_digit(squares_without_piece, 10).unwrap());
                squares_without_piece = 0;
            }

            let mut piece_symbol = text::piece_to_symbol(piece).unwrap();
            if (board.pieces[WHITE][piece] & (1u64 << square)) == 0 {
                piece_symbol = piece_symbol.to_lowercase().collect::<Vec<char>>()[0];
            }

            result.push(piece_symbol);
        }

        if (square % 8) == 0 {
            if squares_without_piece != 0 {
                result.push(char::from_digit(squares_without_piece, 10).unwrap());
                squares_without_piece = 0;
            }

            if square != 0 {
                result.push('/');
            }
        }
    }

    result
}

/// Parses FEN's active color and stores it into the `board`. Returns [Err] with the proper error message if `active_color` couldn't be parsed.
fn fen_to_active_color(board: &mut Board, color: &str) -> Result<(), String> {
    let color_char = color.chars().next().ok_or(format!("Invalid FEN, bad color: color={}", color))?;
    board.active_color = match color_char {
        'w' => WHITE,
        'b' => BLACK,
        _ => return Err(format!("Invalid FEN, bad color: color={}", color)),
    };

    Ok(())
}

/// Converts active color from the `board` into the FEN chunk.
fn active_color_to_fen(board: &Board) -> String {
    match board.active_color {
        WHITE => "w".to_string(),
        BLACK => "b".to_string(),
        _ => panic!("Invalid value: board.active_color={}", board.active_color),
    }
}

/// Parses FEN's castling rights and stores them into the `board`. Returns [Err] with the proper error message if `castling` couldn't be parsed.
fn fen_to_castling(board: &mut Board, castling: &str) -> Result<(), String> {
    if castling == "-" {
        return Ok(());
    }

    for right in castling.chars() {
        board.castling_rights |= match right {
            'K' => CastlingRights::WHITE_SHORT_CASTLING,
            'Q' => CastlingRights::WHITE_LONG_CASTLING,
            'k' => CastlingRights::BLACK_SHORT_CASTLING,
            'q' => CastlingRights::BLACK_LONG_CASTLING,
            _ => return Err(format!("Invalid FEN, bad castling rights: castling={}", castling)),
        };
    }

    Ok(())
}

/// Converts castling rights from the `board` into the FEN chunk.
fn castling_to_fen(board: &Board) -> String {
    if board.castling_rights == CastlingRights::NONE {
        return "-".to_string();
    }

    let mut result = String::new();

    if board.castling_rights.contains(CastlingRights::WHITE_SHORT_CASTLING) {
        result.push('K');
    }
    if board.castling_rights.contains(CastlingRights::WHITE_LONG_CASTLING) {
        result.push('Q');
    }
    if board.castling_rights.contains(CastlingRights::BLACK_SHORT_CASTLING) {
        result.push('k');
    }
    if board.castling_rights.contains(CastlingRights::BLACK_LONG_CASTLING) {
        result.push('q');
    }

    result
}

/// Parses FEN's en passant and stores it into the `board`. Returns [Err] with the proper error message if `en_passant` couldn't be parsed.
fn fen_to_en_passant(board: &mut Board, en_passant: &str) -> Result<(), String> {
    if en_passant == "-" {
        return Ok(());
    }

    let mut chars = en_passant.chars();
    let file = chars.next().ok_or(format!("Invalid FEN, bad en passant file: en_passant={}", en_passant))? as u8;
    let rank = chars.next().ok_or(format!("Invalid FEN, bad en passant rank: en_passant={}", en_passant))? as u8;

    let square = (7 - (file - b'a')) + 8 * (rank - b'1');
    board.en_passant = 1u64 << square;

    Ok(())
}

/// Converts en passant from the `board` into the FEN chunk.
fn en_passant_to_fen(board: &Board) -> String {
    if board.en_passant == 0 {
        return "-".to_string();
    }

    let square = board.en_passant.bit_scan();
    let file = square % 8;
    let rank = square / 8;

    let result = vec![char::from(b'a' + (7 - file) as u8), char::from(b'1' + rank as u8)];
    result.into_iter().collect()
}

/// Parses FEN's halfmove clock and stores it into the `board`. Returns [Err] with the proper error message if `halfmove_clock` couldn't be parsed.
fn fen_to_halfmove_clock(board: &mut Board, halfmove_clock: &str) -> Result<(), String> {
    board.halfmove_clock = match halfmove_clock.parse::<u16>() {
        Ok(value) => value,
        Err(error) => return Err(format!("Invalid FEN, bad halfmove clock: {}", error)),
    };

    Ok(())
}

/// Converts halfmove clock from the `board` into the FEN chunk.
fn halfmove_clock_to_fen(board: &Board) -> String {
    board.halfmove_clock.to_string()
}

/// Parses FEN's fullmove number and stores it into the `board`. Returns [Err] with the proper error message if `fullmove_number` couldn't be parsed.
fn fen_to_fullmove_number(board: &mut Board, fullmove_number: &str) -> Result<(), String> {
    board.fullmove_number = match fullmove_number.parse::<u16>() {
        Ok(value) => value,
        Err(error) => return Err(format!("Invalid FEN, bad fullmove clock: {}", error)),
    };

    Ok(())
}

/// Converts fullmove number from the `board` into the FEN chunk.
fn fullmove_number_to_fen(board: &Board) -> String {
    board.fullmove_number.to_string()
}
