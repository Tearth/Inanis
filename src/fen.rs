use crate::{board::*, common::*};

pub fn fen_to_board(fen: &str) -> Result<Bitboard, &str> {
    let parts: Vec<&str> = fen.split(' ').collect();
    if parts.len() < 6 {
        return Err("Invalid FEN");
    }

    let mut board = Bitboard::new();
    fen_to_pieces(&mut board, parts[0].trim())?;
    fen_to_active_color(&mut board, parts[1].trim())?;
    fen_to_castling(&mut board, parts[2].trim())?;
    fen_to_en_passant(&mut board, parts[3].trim())?;
    fen_to_halfmove_clock(&mut board, parts[4].trim())?;
    fen_to_fullmove_number(&mut board, parts[5].trim())?;

    Ok(board)
}

pub fn board_to_fen(board: &Bitboard) -> Result<String, &str> {
    Err("Not implemented")
}

fn fen_to_pieces<'a>(board: &mut Bitboard, pieces: &str) -> Result<(), &'a str> {
    let mut current_field_index = 63;
    for char in pieces.chars() {
        if char == '/' {
            continue;
        } else if char.is_digit(10) {
            current_field_index -= char.to_digit(10).unwrap() as i32;
        } else {
            let color = if char.is_uppercase() { WHITE } else { BLACK };
            let piece = match char {
                'p' | 'P' => PAWN,
                'n' | 'N' => KNIGHT,
                'b' | 'B' => BISHOP,
                'r' | 'R' => ROOK,
                'q' | 'Q' => QUEEN,
                'k' | 'K' => KING,
                _ => return Err("Invalid FEN"),
            };

            board.add_piece(current_field_index as u8, piece, color);
            current_field_index -= 1;
        }
    }

    Ok(())
}

fn fen_to_active_color<'a>(board: &mut Bitboard, active_color: &str) -> Result<(), &'a str> {
    let color_char = active_color.chars().next().unwrap();
    board.color_to_move = if color_char == 'w' { WHITE } else { BLACK };

    Ok(())
}

fn fen_to_castling<'a>(board: &mut Bitboard, castling: &str) -> Result<(), &'a str> {
    if castling == "-" {
        return Ok(());
    }

    for right in castling.chars() {
        board.castling_rights |= match right {
            'K' => CastlingRights::WHITE_SHORT_CASTLING,
            'Q' => CastlingRights::WHITE_LONG_CASTLING,
            'k' => CastlingRights::BLACK_SHORT_CASTLING,
            'q' => CastlingRights::BLACK_LONG_CASTLING,
            _ => CastlingRights::NONE,
        };
    }

    Ok(())
}

fn fen_to_en_passant<'a>(board: &mut Bitboard, en_passant: &str) -> Result<(), &'a str> {
    if en_passant == "-" {
        return Ok(());
    }

    let mut chars = en_passant.chars();
    let field_index = (7 - ((chars.next().unwrap() as u8) - b'a')) + 8 * ((chars.next().unwrap() as u8) - b'1');
    board.en_passant = 1u64 << field_index;

    Ok(())
}

fn fen_to_halfmove_clock<'a>(board: &mut Bitboard, halfmove_clock: &str) -> Result<(), &'a str> {
    board.halfmove_clock = match halfmove_clock.parse::<u32>() {
        Ok(value) => value,
        Err(_) => return Err("Invalid FEN"),
    };

    Ok(())
}

fn fen_to_fullmove_number<'a>(board: &mut Bitboard, fullmove_number: &str) -> Result<(), &'a str> {
    board.fullmove_number = match fullmove_number.parse::<u32>() {
        Ok(value) => value,
        Err(_) => return Err("Invalid FEN"),
    };

    Ok(())
}
