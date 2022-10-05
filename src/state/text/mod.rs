use super::*;

pub mod fen;
pub mod moves;
pub mod pgn;

/// Converts piece `symbol` (p/P, n/N, b/B, r/R, q/Q, k/K) into the corresponding [u8] value. Returns [Err] with the proper error messages when the `symbol` is unknown.
pub fn symbol_to_piece(symbol: char) -> Result<u8, String> {
    match symbol {
        'p' | 'P' => Ok(PAWN),
        'n' | 'N' => Ok(KNIGHT),
        'b' | 'B' => Ok(BISHOP),
        'r' | 'R' => Ok(ROOK),
        'q' | 'Q' => Ok(QUEEN),
        'k' | 'K' => Ok(KING),
        _ => Err(format!("Invalid parameter: symbol={}", symbol)),
    }
}

/// Converts `piece` into the corresponding character (p/P, n/N, b/B, r/R, q/Q, k/K). Returns [Err] with the proper error message when the `piece` is unknown.
pub fn piece_to_symbol(piece: u8) -> Result<char, String> {
    match piece {
        PAWN => Ok('P'),
        KNIGHT => Ok('N'),
        BISHOP => Ok('B'),
        ROOK => Ok('R'),
        QUEEN => Ok('Q'),
        KING => Ok('K'),
        _ => Err(format!("Invalid parameter: piece={}", piece)),
    }
}
