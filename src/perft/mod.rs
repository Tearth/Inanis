use self::context::PerftContext;
use crate::state::movescan::Move;
use std::mem::MaybeUninit;
use std::u64;

pub mod context;
pub mod divided;
pub mod fast;
pub mod normal;

#[macro_export]
macro_rules! run_perft {
    ($color:expr, $context:expr, $depth:expr, $invert:expr) => {
        match $invert {
            true => match $color {
                crate::state::WHITE => crate::perft::run::<{ crate::state::BLACK }>($context, $depth),
                crate::state::BLACK => crate::perft::run::<{ crate::state::WHITE }>($context, $depth),
                _ => panic!("Invalid value: $color={}", $color),
            },
            false => match $color {
                crate::state::WHITE => crate::perft::run::<{ crate::state::WHITE }>($context, $depth),
                crate::state::BLACK => crate::perft::run::<{ crate::state::BLACK }>($context, $depth),
                _ => panic!("Invalid value: $color={}", $color),
            },
        }
    };
}

pub fn run<const COLOR: u8>(context: &mut PerftContext, depth: i32) -> u64 {
    if context.check_integrity {
        let original_hash = context.board.hash;
        let original_evaluation = context.board.evaluate();

        context.board.recalculate_hash();
        context.board.recalculate_incremental_values();

        if original_hash != context.board.hash {
            panic!("Integrity check failed: invalid hash");
        }

        if original_evaluation != context.board.evaluate() {
            panic!("Integrity check failed: invalid evaluation")
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
            count += run_perft!(COLOR, context, depth - 1, true);
        }

        context.board.undo_move::<COLOR>(r#move);
    }

    if context.fast {
        context.hashtable.add(context.board.hash, depth as u8, count);
    }

    count
}
