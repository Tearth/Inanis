use super::board::Bitboard;
use super::board::CastlingRights;
use super::*;

pub fn fen_to_board(fen: &str) -> Result<Bitboard, &'static str> {
    let tokens: Vec<&str> = fen.split(' ').map(|v| v.trim()).collect();
    if tokens.len() < 4 {
        return Err("Invalid FEN: input too short");
    }

    let mut board = Default::default();
    fen_to_pieces(&mut board, tokens[0])?;
    fen_to_active_color(&mut board, tokens[1])?;
    fen_to_castling(&mut board, tokens[2])?;
    fen_to_en_passant(&mut board, tokens[3])?;

    // Ignore halfmove clock and fullmove number if not present (EPD)
    let _ = fen_to_halfmove_clock(&mut board, tokens[4]);
    let _ = fen_to_fullmove_number(&mut board, tokens[5]);

    board.recalculate_hash();
    board.recalculate_pawn_hash();
    board.recalculate_incremental_values();

    Ok(board)
}

pub fn board_to_fen(board: &Bitboard) -> String {
    let pieces = pieces_to_fen(board);
    let active_color = active_color_to_fen(board);
    let castling = castling_to_fen(board);
    let en_passant = en_passant_to_fen(board);
    let halfmove_clock = halfmove_clock_to_fen(board);
    let fullmove_number = fullmove_number_to_fen(board);

    format!(
        "{} {} {} {} {} {}",
        pieces, active_color, castling, en_passant, halfmove_clock, fullmove_number
    )
}

fn fen_to_pieces(board: &mut Bitboard, pieces: &str) -> Result<(), &'static str> {
    let mut current_field_index = 63;
    for char in pieces.chars().filter(|&x| x != '/') {
        if char.is_digit(10) {
            current_field_index -= char.to_digit(10).ok_or("Invalid FEN: bad piece")? as i32;
        } else {
            let color = if char.is_uppercase() { WHITE } else { BLACK };
            let piece = symbol_to_piece(char)?;

            board.add_piece(color, piece, current_field_index as u8);
            current_field_index -= 1;
        }
    }

    Ok(())
}

fn pieces_to_fen(board: &Bitboard) -> String {
    let mut result = String::new();
    let mut fields_without_piece = 0;

    for field_index in (0..64).rev() {
        let piece = board.get_piece(field_index);
        if piece == u8::MAX {
            fields_without_piece += 1;
        } else {
            if fields_without_piece != 0 {
                result.push(char::from_digit(fields_without_piece, 10).unwrap());
                fields_without_piece = 0;
            }

            let mut piece_symbol = piece_to_symbol(piece).unwrap();
            if (board.pieces[WHITE as usize][piece as usize] & (1u64 << field_index)) == 0 {
                piece_symbol = piece_symbol.to_lowercase().collect::<Vec<char>>()[0];
            }

            result.push(piece_symbol);
        }

        if (field_index % 8) == 0 {
            if fields_without_piece != 0 {
                result.push(char::from_digit(fields_without_piece, 10).unwrap());
                fields_without_piece = 0;
            }

            if field_index != 0 {
                result.push('/');
            }
        }
    }

    result
}

fn fen_to_active_color(board: &mut Bitboard, active_color: &str) -> Result<(), &'static str> {
    let color_char = active_color.chars().next().ok_or("Invalid FEN: bad active color")?;
    board.active_color = if color_char == 'w' { WHITE } else { BLACK };

    Ok(())
}

fn active_color_to_fen(board: &Bitboard) -> String {
    match board.active_color {
        WHITE => "w".to_string(),
        BLACK => "b".to_string(),
        _ => panic!("Invalid value: board.active_color={}", board.active_color),
    }
}

fn fen_to_castling(board: &mut Bitboard, castling: &str) -> Result<(), &'static str> {
    if castling == "-" {
        return Ok(());
    }

    for right in castling.chars() {
        board.castling_rights |= match right {
            'K' => CastlingRights::WHITE_SHORT_CASTLING,
            'Q' => CastlingRights::WHITE_LONG_CASTLING,
            'k' => CastlingRights::BLACK_SHORT_CASTLING,
            'q' => CastlingRights::BLACK_LONG_CASTLING,
            _ => return Err("Invalid FEN: bad castling rights"),
        };
    }

    Ok(())
}

fn castling_to_fen(board: &Bitboard) -> String {
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

fn fen_to_en_passant(board: &mut Bitboard, en_passant: &str) -> Result<(), &'static str> {
    if en_passant == "-" {
        return Ok(());
    }

    let mut chars = en_passant.chars();
    let file = chars.next().ok_or("Invalid FEN: bad en passant file")? as u8;
    let rank = chars.next().ok_or("Invalid FEN: bad en passant rank")? as u8;

    let field_index = (7 - (file - b'a')) + 8 * (rank - b'1');
    board.en_passant = 1u64 << field_index;

    Ok(())
}

fn en_passant_to_fen(board: &Bitboard) -> String {
    if board.en_passant == 0 {
        return "-".to_string();
    }

    let field_index = bit_scan(board.en_passant);
    let file = field_index % 8;
    let rank = field_index / 8;

    let result = vec![char::from(b'a' + (7 - file)), char::from(b'1' + rank)];
    result.into_iter().collect()
}

fn fen_to_halfmove_clock(board: &mut Bitboard, halfmove_clock: &str) -> Result<(), &'static str> {
    board.halfmove_clock = match halfmove_clock.parse::<u16>() {
        Ok(value) => value,
        Err(_) => return Err("Invalid FEN: bad halfmove clock"),
    };

    Ok(())
}

fn halfmove_clock_to_fen(board: &Bitboard) -> String {
    board.halfmove_clock.to_string()
}

fn fen_to_fullmove_number(board: &mut Bitboard, fullmove_number: &str) -> Result<(), &'static str> {
    board.fullmove_number = match fullmove_number.parse::<u16>() {
        Ok(value) => value,
        Err(_) => return Err("Invalid FEN: bad fullmove flock"),
    };

    Ok(())
}

fn fullmove_number_to_fen(board: &Bitboard) -> String {
    board.fullmove_number.to_string()
}
