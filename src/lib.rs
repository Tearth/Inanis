#![allow(
    clippy::needless_range_loop,
    clippy::identity_op,
    clippy::let_and_return,
    clippy::uninit_assumed_init,
    clippy::nonminimal_bool,
    clippy::collapsible_if
)]

#[macro_use]
extern crate bitflags;

pub mod benchmark;
pub mod bit;
pub mod board;
pub mod clock;
pub mod common;
pub mod evaluation;
pub mod fen;
pub mod movegen;
pub mod movescan;
pub mod patterns;
pub mod perft;
pub mod qsearch;
pub mod search;
pub mod terminal;
pub mod uci;
pub mod zobrist;
