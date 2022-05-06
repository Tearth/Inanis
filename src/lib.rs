#![allow(
    unused_assignments,
    clippy::needless_range_loop,
    clippy::identity_op,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    clippy::if_same_then_else,
    clippy::declare_interior_mutable_const
)]

//! The main page of the Inanis documentation. Feel free to explore it by going into the specific module below,
//! or by clicking "See all inanis's items" on the left to see every possible item.
//!
//! Homepage: <https://github.com/Tearth/Inanis>

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
}
