use crate::cache::perft::PerftHashTable;
use crate::engine::*;
use crate::perft::context::PerftContext;
use crate::state::board::Bitboard;
use crate::state::movescan::Move;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::u64;

use super::run_internal;

pub fn run(depth: i32, board: &mut Bitboard, hashtable_size: usize, threads_count: usize) -> (u64, f32) {
    let queue = Arc::new(Mutex::new(Vec::new()));
    let hashtable = Arc::new(PerftHashTable::new(hashtable_size));
    let mut threads = Vec::new();

    let mut moves: [Move; MAX_MOVES_COUNT] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = board.get_all_moves(&mut moves, u64::MAX);

    for r#move in &moves[0..moves_count] {
        let mut cloned_board = board.clone();
        cloned_board.make_move(r#move);

        queue.lock().unwrap().push(cloned_board);
    }

    for _ in 0..threads_count {
        let queue_arc = queue.clone();
        let hashtable_arc = hashtable.clone();

        threads.push(thread::spawn(move || {
            let mut count = 0;
            let mut hashtable_usage = 0.0;

            loop {
                let mut board = {
                    match queue_arc.lock().unwrap().pop() {
                        Some(value) => value,
                        None => break,
                    }
                };

                let mut context = PerftContext::new(&mut board, &hashtable_arc, false, true);
                count += run_internal(&mut context, depth - 1);

                hashtable_usage = context.hashtable.get_usage();
            }

            (count, hashtable_usage)
        }));
    }

    let mut total_count = 0;
    let mut hashtable_usage_accumulator = 0.0;

    for thread in threads {
        let (count, hashtable_usage) = thread.join().unwrap();

        total_count += count;
        hashtable_usage_accumulator += hashtable_usage;
    }

    (total_count, hashtable_usage_accumulator / (threads_count as f32))
}
