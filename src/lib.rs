#![allow(
    clippy::needless_range_loop,
    clippy::identity_op,
    clippy::let_and_return,
    clippy::uninit_assumed_init,
    clippy::nonminimal_bool,
    clippy::collapsible_if,
    clippy::single_char_add_str,
    clippy::too_many_arguments
)]

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

pub fn init() {
    fastrand::seed(584578);

    pst::init();
    see::init();
    patterns::init();
    movegen::init();
    zobrist::init();
    evaluation::init();
}
