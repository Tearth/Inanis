#![allow(
    unused_assignments,
    clippy::needless_range_loop,
    clippy::identity_op,
    clippy::let_and_return,
    clippy::uninit_assumed_init,
    clippy::nonminimal_bool,
    clippy::collapsible_if,
    clippy::single_char_add_str,
    clippy::too_many_arguments,
    clippy::if_same_then_else
)]

//! Main page of the Inanis documentation. Feel free to explore it by going into the specific module below,
//! or by clicking "See all inanis's items" on the left to see every possible item.
//!
//! Homepage: https://github.com/Tearth/Inanis

use engine::see;
use evaluation::pst;
use state::movegen;
use state::patterns;
use state::zobrist;

pub mod cache;
pub mod engine;
pub mod evaluation;
pub mod interface;
pub mod perft;
pub mod state;
pub mod utils;

#[macro_use]
extern crate bitflags;

/// Initializes all engine's components (random seed, PST, SEE, patterns, move generator, Zobrist hashing and evaluation), to make it ready to run.
pub fn init() {
    fastrand::seed(584578);

    pst::init();
    see::init();
    patterns::init();
    movegen::init();
    zobrist::init();
    evaluation::init();
}
