use crate::board::Bitboard;
use crate::common::*;
use crate::movescan::Move;
use std::mem::{size_of, MaybeUninit};
use std::sync::{Arc, Mutex};
use std::{thread, u64};

const BUCKET_SLOTS: usize = 4;

struct PerftContext<'a> {
    pub board: &'a mut Bitboard,
    pub check_integrity: bool,
    pub fast: bool,
    pub hash_table: &'a mut PerftHashTable,
}

impl<'a> PerftContext<'a> {
    pub fn new(board: &'a mut Bitboard, check_integrity: bool, fast: bool, hash_table: &'a mut PerftHashTable) -> PerftContext<'a> {
        PerftContext {
            board,
            check_integrity,
            fast,
            hash_table,
        }
    }
}

struct PerftHashTable {
    table: Vec<PerftHashTableBucket>,
    slots: usize,
}

impl PerftHashTable {
    fn new(size: usize) -> PerftHashTable {
        let buckets = size / size_of::<PerftHashTableBucket>();
        let mut hashtable = PerftHashTable {
            table: Vec::with_capacity(buckets),
            slots: buckets,
        };

        if size != 0 {
            hashtable.table.resize(hashtable.slots, PerftHashTableBucket::new());
        }

        hashtable
    }

    pub fn add(&mut self, hash: u64, depth: u8, leafs_count: u64) {
        let mut bucket = self.table[(hash as usize) % self.slots];
        let mut smallest_depth = (bucket.entries[0].key_and_depth & 0xf) as u8;
        let mut smallest_depth_index = 0;

        for entry_index in 1..BUCKET_SLOTS {
            let entry_depth = (bucket.entries[entry_index].key_and_depth & 0xf) as u8;
            if entry_depth <= smallest_depth {
                smallest_depth = entry_depth;
                smallest_depth_index = entry_index;
            }
        }

        bucket.entries[smallest_depth_index] = PerftHashTableEntry::new(hash, depth, leafs_count);
        self.table[(hash as usize) % self.slots] = bucket;
    }

    pub fn get(&self, hash: u64, depth: u8) -> Option<PerftHashTableEntry> {
        let bucket = self.table[(hash as usize) % self.slots];
        for entry in bucket.entries {
            if entry.key_and_depth == ((hash & !0xf) | (depth as u64)) {
                return Some(entry);
            }
        }

        None
    }

    pub fn get_usage(&self) -> f32 {
        let mut filled_entries = 0;
        let buckets_count_to_check = 1000 / BUCKET_SLOTS;

        for bucket_index in 0..buckets_count_to_check {
            for entry in self.table[bucket_index].entries {
                if entry.key_and_depth != 0 && entry.leafs_count != 0 {
                    filled_entries += 1;
                }
            }
        }

        ((filled_entries as f32) / 1000.0) * 100.0
    }
}

#[repr(align(64))]
#[derive(Clone, Copy)]
struct PerftHashTableBucket {
    pub entries: [PerftHashTableEntry; BUCKET_SLOTS],
}

impl PerftHashTableBucket {
    fn new() -> PerftHashTableBucket {
        PerftHashTableBucket {
            entries: [PerftHashTableEntry::new(0, 0, 0); BUCKET_SLOTS],
        }
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
    let mut hash_table = PerftHashTable::new(0);
    let mut context = PerftContext::new(board, check_integrity, false, &mut hash_table);
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

    let mut hash_table = PerftHashTable::new(0);
    let mut context = PerftContext::new(board, check_integrity, false, &mut hash_table);
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

pub fn run_fast(
    depth: i32,
    board: &mut Bitboard,
    check_integrity: bool,
    hashtable_size: usize,
    threads_count: usize,
) -> Result<(u64, f32), &'static str> {
    let queue = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_moves_active_color(&mut moves);

    for r#move in &moves[0..moves_count] {
        let mut cloned_board = board.clone();
        cloned_board.make_move_active_color(r#move);

        queue.lock().unwrap().push(cloned_board);
    }

    for _ in 0..threads_count {
        let cloned_queue = queue.clone();
        threads.push(thread::spawn(move || {
            let mut count = 0;
            let mut hash_table_usage = 0.0;
            let mut hash_table = PerftHashTable::new(hashtable_size / threads_count);

            loop {
                let mut board = {
                    let mut lock = cloned_queue.lock().unwrap();
                    match lock.pop() {
                        Some(value) => value,
                        None => break,
                    }
                };

                let mut context = PerftContext::new(&mut board, check_integrity, true, &mut hash_table);
                count += match context.board.active_color {
                    WHITE => run_internal::<WHITE, BLACK>(&mut context, depth - 1),
                    BLACK => run_internal::<BLACK, WHITE>(&mut context, depth - 1),
                    _ => panic!("Invalid value: board.active_color={}", context.board.active_color),
                };

                hash_table_usage = context.hash_table.get_usage();
            }

            (count, hash_table_usage)
        }));
    }

    let mut total_count = 0;
    let mut hash_table_usage_accumulator = 0.0;

    for thread in threads {
        let (count, hash_table_usage) = thread.join().unwrap();
        total_count += count;
        hash_table_usage_accumulator += hash_table_usage;
    }

    Ok((total_count, hash_table_usage_accumulator / (threads_count as f32)))
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
        if let Some(entry) = context.hash_table.get(context.board.hash, depth as u8) {
            return entry.leafs_count;
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
