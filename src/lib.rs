#![allow(
    clippy::needless_range_loop,
    clippy::identity_op,
    clippy::let_and_return,
    clippy::uninit_assumed_init
)]

#[macro_use]
extern crate bitflags;

pub mod board;
pub mod common;
pub mod helpers;
pub mod movegen;
pub mod movescan;
pub mod patterns;
pub mod perft;
pub mod terminal;
