use crate::{board::*, common::*, helpers::*, movegen::*};

pub enum MoveFlags {
    Quiet,
    DoublePush,
    KingCastle,
    QueenCastle,
    Capture,
    EnPassant,
    Undefined1,
    Undefined2,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    KnightPromotionCapture,
    BishopPromotionCapture,
    RookPromotionCapture,
    QueenPromotionCapture,
}

#[derive(Copy, Clone)]
pub struct Move {
    pub data: u16,
}

impl Move {
    pub fn new(from: u8, to: u8, flags: MoveFlags) -> Move {
        Move {
            data: ((flags as u16) << 12) | ((to as u16) << 6) | (from as u16),
        }
    }

    pub fn get_from(&self) -> u8 {
        (self.data & 0x3f) as u8
    }

    pub fn get_to(&self) -> u8 {
        ((self.data >> 6) & 0x3f) as u8
    }

    pub fn get_flags(&self) -> MoveFlags {
        match (self.data >> 12) & 0xf {
            0 => MoveFlags::Quiet,
            1 => MoveFlags::DoublePush,
            2 => MoveFlags::KingCastle,
            3 => MoveFlags::QueenCastle,
            4 => MoveFlags::Capture,
            5 => MoveFlags::EnPassant,
            6 => MoveFlags::Undefined1,
            7 => MoveFlags::Undefined2,
            8 => MoveFlags::KnightPromotion,
            9 => MoveFlags::BishopPromotion,
            10 => MoveFlags::RookPromotion,
            11 => MoveFlags::QueenPromotion,
            12 => MoveFlags::KnightPromotionCapture,
            13 => MoveFlags::BishopPromotionCapture,
            14 => MoveFlags::RookPromotionCapture,
            15 => MoveFlags::QueenPromotionCapture,
            _ => panic!("Invalid move flag"),
        }
    }
}

pub fn scan_knight_moves(board: &Bitboard, color: u8, moves: &mut [Move], mut index: usize) -> usize {
    let mut pieces = board.pieces[color as usize][KNIGHT as usize];
    let enemy_color = match color {
        WHITE => BLACK,
        BLACK => WHITE,
        _ => u8::MAX,
    };

    while pieces != 0 {
        let from_field = get_lsb(pieces);
        let from_field_index = bit_scan(from_field);
        pieces = pop_lsb(pieces);

        let mut piece_moves = get_knight_moves(from_field_index as usize) & !board.occupancy[color as usize];
        while piece_moves != 0 {
            let to_field = get_lsb(piece_moves);
            let to_field_index = bit_scan(to_field);
            piece_moves = pop_lsb(piece_moves);

            let mut flags = MoveFlags::Quiet;
            if (to_field & board.occupancy[enemy_color as usize]) != 0 {
                flags = MoveFlags::Capture;
            }

            moves[index] = Move::new(from_field_index as u8, to_field_index as u8, flags);
            index += 1;
        }
    }

    index
}

pub fn scan_bishop_moves(board: &Bitboard, color: Color, moves: &mut [Move], mut index: usize) -> usize {
    let mut pieces = board.pieces[color as usize][BISHOP as usize];
    let enemy_color = match color {
        WHITE => BLACK,
        BLACK => WHITE,
        _ => u8::MAX,
    };

    while pieces != 0 {
        let from_field = get_lsb(pieces);
        let from_field_index = bit_scan(from_field);
        pieces = pop_lsb(pieces);

        let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
        let mut piece_moves = get_bishop_moves(occupancy, from_field_index as usize) & !board.occupancy[color as usize];
        while piece_moves != 0 {
            let to_field = get_lsb(piece_moves);
            let to_field_index = bit_scan(to_field);
            piece_moves = pop_lsb(piece_moves);

            let mut flags = MoveFlags::Quiet;
            if (to_field & board.occupancy[enemy_color as usize]) != 0 {
                flags = MoveFlags::Capture;
            }

            moves[index] = Move::new(from_field_index as u8, to_field_index as u8, flags);
            index += 1;
        }
    }

    index
}

pub fn scan_rook_moves(board: &Bitboard, color: Color, moves: &mut [Move], mut index: usize) -> usize {
    let mut pieces = board.pieces[color as usize][ROOK as usize];
    let enemy_color = match color {
        WHITE => BLACK,
        BLACK => WHITE,
        _ => u8::MAX,
    };

    while pieces != 0 {
        let from_field = get_lsb(pieces);
        let from_field_index = bit_scan(from_field);
        pieces = pop_lsb(pieces);

        let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
        let mut piece_moves = get_rook_moves(occupancy, from_field_index as usize) & !board.occupancy[color as usize];
        while piece_moves != 0 {
            let to_field = get_lsb(piece_moves);
            let to_field_index = bit_scan(to_field);
            piece_moves = pop_lsb(piece_moves);

            let mut flags = MoveFlags::Quiet;
            if (to_field & board.occupancy[enemy_color as usize]) != 0 {
                flags = MoveFlags::Capture;
            }

            moves[index] = Move::new(from_field_index as u8, to_field_index as u8, flags);
            index += 1;
        }
    }

    index
}

pub fn scan_queen_moves(board: &Bitboard, color: Color, moves: &mut [Move], mut index: usize) -> usize {
    let mut pieces = board.pieces[color as usize][QUEEN as usize];
    let enemy_color = match color {
        WHITE => BLACK,
        BLACK => WHITE,
        _ => u8::MAX,
    };

    while pieces != 0 {
        let from_field = get_lsb(pieces);
        let from_field_index = bit_scan(from_field);
        pieces = pop_lsb(pieces);

        let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
        let mut piece_moves = get_queen_moves(occupancy, from_field_index as usize) & !board.occupancy[color as usize];
        while piece_moves != 0 {
            let to_field = get_lsb(piece_moves);
            let to_field_index = bit_scan(to_field);
            piece_moves = pop_lsb(piece_moves);

            let mut flags = MoveFlags::Quiet;
            if (to_field & board.occupancy[enemy_color as usize]) != 0 {
                flags = MoveFlags::Capture;
            }

            moves[index] = Move::new(from_field_index as u8, to_field_index as u8, flags);
            index += 1;
        }
    }

    index
}

pub fn scan_king_moves(board: &Bitboard, color: Color, moves: &mut [Move], mut index: usize) -> usize {
    let mut pieces = board.pieces[color as usize][KING as usize];
    let enemy_color = match color {
        WHITE => BLACK,
        BLACK => WHITE,
        _ => u8::MAX,
    };

    while pieces != 0 {
        let from_field = get_lsb(pieces);
        let from_field_index = bit_scan(from_field);
        pieces = pop_lsb(pieces);

        let mut piece_moves = get_king_moves(from_field_index as usize) & !board.occupancy[color as usize];
        while piece_moves != 0 {
            let to_field = get_lsb(piece_moves);
            let to_field_index = bit_scan(to_field);
            piece_moves = pop_lsb(piece_moves);

            let mut flags = MoveFlags::Quiet;
            if (to_field & board.occupancy[enemy_color as usize]) != 0 {
                flags = MoveFlags::Capture;
            }

            moves[index] = Move::new(from_field_index as u8, to_field_index as u8, flags);
            index += 1;
        }
    }

    index
}

pub fn scan_pawn_moves(board: &Bitboard, color: Color, moves: &mut [Move], mut index: usize) -> usize {
    index = scan_pawn_moves_single_push(board, color, moves, index);
    index = scan_pawn_moves_double_push(board, color, moves, index);

    let left_shift = if color == WHITE { 9 } else { 7 };
    let right_shift = if color == WHITE { 7 } else { 9 };
    index = scan_pawn_moves_diagonal_attacks(board, color, left_shift, FILE_A, moves, index);
    index = scan_pawn_moves_diagonal_attacks(board, color, right_shift, FILE_H, moves, index);

    index
}

fn scan_pawn_moves_single_push(board: &Bitboard, color: Color, moves: &mut [Move], mut index: usize) -> usize {
    let pieces = board.pieces[color as usize][PAWN as usize];

    let shift: i8;
    let mut target_fields: u64;

    let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
    if color == WHITE {
        shift = 8;
        target_fields = (pieces << 8) & !occupancy;
    } else {
        shift = -8;
        target_fields = (pieces >> 8) & !occupancy;
    }

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        target_fields = pop_lsb(target_fields);

        moves[index] = Move::new(
            ((to_field_index as i8) - shift) as u8,
            to_field_index as u8,
            MoveFlags::Quiet,
        );
        index += 1;
    }

    index
}

fn scan_pawn_moves_double_push(board: &Bitboard, color: Color, moves: &mut [Move], mut index: usize) -> usize {
    let pieces = board.pieces[color as usize][PAWN as usize];
    let enemy_color = match color {
        WHITE => BLACK,
        BLACK => WHITE,
        _ => u8::MAX,
    };

    let shift: i8;
    let mut target_fields: u64;

    let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
    if color == WHITE {
        shift = 16;
        target_fields = ((pieces & RANK_B) << 8) & !occupancy;
        target_fields = (target_fields << 8) & !occupancy;
    } else {
        shift = -16;
        target_fields = ((pieces & RANK_G) >> 8) & !occupancy;
        target_fields = (target_fields >> 8) & !occupancy;
    }

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        target_fields = pop_lsb(target_fields);

        moves[index] = Move::new(
            ((to_field_index as i8) - shift) as u8,
            to_field_index as u8,
            MoveFlags::DoublePush,
        );
        index += 1;
    }

    index
}

fn scan_pawn_moves_diagonal_attacks(
    board: &Bitboard,
    color: Color,
    direction: i8,
    forbidden_file: u64,
    moves: &mut [Move],
    mut index: usize,
) -> usize {
    let pieces = board.pieces[color as usize][PAWN as usize];
    let enemy_color = match color {
        WHITE => BLACK,
        BLACK => WHITE,
        _ => u8::MAX,
    };

    let shift: i8;
    let mut target_fields: u64;

    if color == WHITE {
        shift = direction;
        target_fields = ((pieces & !forbidden_file) << direction) & board.occupancy[enemy_color as usize];
    } else {
        shift = -direction;
        target_fields = ((pieces & !forbidden_file) >> direction) & board.occupancy[enemy_color as usize];
    }

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        target_fields = pop_lsb(target_fields);

        moves[index] = Move::new(
            ((to_field_index as i8) - shift) as u8,
            to_field_index as u8,
            MoveFlags::Capture,
        );
        index += 1;
    }

    index
}
