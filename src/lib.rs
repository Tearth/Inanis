#![allow(
    unused_assignments,
    clippy::needless_range_loop,
    clippy::identity_op,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    clippy::manual_range_patterns,
    clippy::collapsible_else_if
)]

//! The main page of the Inanis documentation. Feel free to explore it by going into the specific module below,
//! or by clicking "See all inanis's items" on the left panel to see every possible item.
//!
//! Homepage: <https://github.com/Tearth/Inanis>

use engine::*;
use state::movescan::Move;
use std::mem::MaybeUninit;

pub mod cache;
pub mod engine;
pub mod evaluation;
pub mod interface;
pub mod perft;
pub mod state;
pub mod tablebases;
pub mod testing;
pub mod tuning;
pub mod utils;

pub type Moves = [MaybeUninit<Move>; MAX_MOVES_COUNT];
pub type MoveScores = [MaybeUninit<i16>; MAX_MOVES_COUNT];
