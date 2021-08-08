use crate::board::common::*;
use crate::board::movescan::Move;
use crate::board::representation::Bitboard;
use crate::board::representation::CastlingRights;
use crate::cache::perft::PerftHashTable;
use std::cell::UnsafeCell;
use std::mem;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::u64;

#[macro_export]
macro_rules! run_internal {
    ($color:expr, $context:expr, $depth:expr, $invert:expr) => {
        match $invert {
            true => match $color {
                WHITE => crate::perft::core::run_internal::<{ crate::board::common::BLACK }>($context, $depth),
                BLACK => crate::perft::core::run_internal::<{ crate::board::common::WHITE }>($context, $depth),
                _ => panic!("Invalid value: $color={}", $color),
            },
            false => match $color {
                WHITE => crate::perft::core::run_internal::<{ crate::board::common::WHITE }>($context, $depth),
                BLACK => crate::perft::core::run_internal::<{ crate::board::common::BLACK }>($context, $depth),
                _ => panic!("Invalid value: $color={}", $color),
            },
        }
    };
}

pub struct PerftContext<'a> {
    pub board: &'a mut Bitboard,
    pub hashtable: &'a Arc<PerftHashTable>,
    pub check_integrity: bool,
    pub fast: bool,
}

impl<'a> PerftContext<'a> {
    pub fn new(board: &'a mut Bitboard, hashtable: &'a Arc<PerftHashTable>, check_integrity: bool, fast: bool) -> PerftContext<'a> {
        PerftContext {
            board,
            hashtable,
            check_integrity,
            fast,
        }
    }
}

pub fn run_internal<const COLOR: u8>(context: &mut PerftContext, depth: i32) -> u64 {
    if context.check_integrity {
        if context.board.hash != context.board.calculate_hash() {
            panic!("Integrity check failed: invalid hash");
        }
    }

    if depth <= 0 {
        return 1;
    }

    if context.fast {
        if let Some(entry) = context.hashtable.get(context.board.hash, depth as u8) {
            return entry.leafs_count;
        }
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves::<COLOR>(&mut moves);

    let mut count = 0;
    for r#move in &moves[0..moves_count] {
        context.board.make_move::<COLOR>(r#move);

        if !context.board.is_king_checked(COLOR) {
            count += run_internal!(COLOR, context, depth - 1, true);
        }

        context.board.undo_move::<COLOR>(r#move);
    }

    if context.fast {
        context.hashtable.add(context.board.hash, depth as u8, count);
    }

    count
}
