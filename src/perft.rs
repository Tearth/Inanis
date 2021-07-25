use crate::board::Bitboard;
use crate::common::*;
use crate::movescan::Move;
use std::mem::{size_of, MaybeUninit};
use std::u64;

struct PerftContext<'a> {
    pub board: &'a mut Bitboard,
    pub check_integrity: bool,
    pub fast: bool,
    pub hash_table: PerftHashTable,
}

impl<'a> PerftContext<'a> {
    pub fn new(board: &mut Bitboard, check_integrity: bool, fast: bool) -> PerftContext {
        PerftContext {
            board,
            check_integrity,
            fast,
            hash_table: PerftHashTable::new(0),
        }
    }
}

struct PerftHashTable {
    table: Vec<PerftHashTableEntry>,
    slots: usize,
}

impl PerftHashTable {
    fn new(size: usize) -> PerftHashTable {
        PerftHashTable {
            table: Vec::new(),
            slots: size / size_of::<PerftHashTableEntry>(),
        }
    }

    pub fn init(&mut self) {
        self.table.resize(self.slots, PerftHashTableEntry::new(0, 0, 0));
    }

    pub fn add(&mut self, hash: u64, depth: u8, leafs_count: u64) {
        self.table[(hash as usize) % self.slots] = PerftHashTableEntry::new(hash, depth, leafs_count);
    }

    pub fn get(&self, hash: u64) -> PerftHashTableEntry {
        self.table[(hash as usize) % self.slots]
    }
}

#[derive(Clone, Copy)]
struct PerftHashTableEntry {
    pub key_and_depth: u64,
    pub leafs_count: u64,
}

impl PerftHashTableEntry {
    fn new(key: u64, depth: u8, leafs_count: u64) -> PerftHashTableEntry {
        PerftHashTableEntry {
            key_and_depth: (key & !0xf) | (depth as u64),
            leafs_count,
        }
    }
}

pub fn run(depth: i32, board: &mut Bitboard, check_integrity: bool) -> Result<u64, &'static str> {
    let mut context = PerftContext::new(board, check_integrity, false);
    let count = match context.board.active_color {
        WHITE => run_internal::<WHITE, BLACK>(&mut context, depth),
        BLACK => run_internal::<BLACK, WHITE>(&mut context, depth),
        _ => panic!("Invalid value: board.active_color={}", board.active_color),
    };

    Ok(count)
}

pub fn run_divided(depth: i32, board: &mut Bitboard, check_integrity: bool) -> Result<Vec<(String, u64)>, &'static str> {
    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_moves_active_color(&mut moves);

    let mut context = PerftContext::new(board, check_integrity, false);
    let mut result = Vec::<(String, u64)>::new();

    for r#move in &moves[0..moves_count] {
        context.board.make_move_active_color(r#move);

        let moves_count = match context.board.active_color {
            WHITE => run_internal::<WHITE, BLACK>(&mut context, depth - 1),
            BLACK => run_internal::<BLACK, WHITE>(&mut context, depth - 1),
            _ => panic!("Invalid value: board.active_color={}", board.active_color),
        };

        result.push((r#move.to_text(), moves_count));
        context.board.undo_move_active_color(r#move);
    }

    Ok(result)
}

pub fn run_fast(depth: i32, board: &mut Bitboard, check_integrity: bool) -> Result<u64, &'static str> {
    let mut context = PerftContext::new(board, check_integrity, true);
    context.hash_table = PerftHashTable::new(1024 * 1024 * 1024);
    context.hash_table.init();

    let count = match context.board.active_color {
        WHITE => run_internal::<WHITE, BLACK>(&mut context, depth),
        BLACK => run_internal::<BLACK, WHITE>(&mut context, depth),
        _ => panic!("Invalid value: board.active_color={}", board.active_color),
    };

    Ok(count)
}

fn run_internal<const COLOR: u8, const ENEMY_COLOR: u8>(context: &mut PerftContext, depth: i32) -> u64 {
    if context.check_integrity {
        if context.board.hash != context.board.calculate_hash() {
            panic!("Integrity check failed: invalid hash");
        }
    }

    if depth <= 0 {
        return 1;
    }

    if context.fast {
        let hash_table_entry = context.hash_table.get(context.board.hash);
        if hash_table_entry.key_and_depth == (context.board.hash & !0xF) | (depth as u64) {
            return hash_table_entry.leafs_count;
        }
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves::<COLOR>(&mut moves);

    let mut count = 0;
    for r#move in &moves[0..moves_count] {
        context.board.make_move::<COLOR>(r#move);

        if !context.board.is_king_checked(COLOR) {
            count += run_internal::<ENEMY_COLOR, COLOR>(context, depth - 1)
        }

        context.board.undo_move::<COLOR>(r#move);
    }

    if context.fast {
        context.hash_table.add(context.board.hash, depth as u8, count);
    }

    count
}
