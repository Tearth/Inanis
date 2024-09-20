use crate::state::*;
use crate::utils::panic_fast;
use pst::*;
use std::ops;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerParameter;

pub mod material;
pub mod mobility;
pub mod params;
pub mod pawns;
pub mod pst;
pub mod safety;

pub const INITIAL_GAME_PHASE: u8 = 24;
pub const PIECE_VALUE: [i16; 6] = [100, 337, 338, 521, 1050, 10000];
pub const PIECE_PHASE_VALUE: [u8; 6] = [0, 1, 1, 2, 4, 0];

macro_rules! s {
    ($opening_score: expr, $ending_score: expr) => {
        PackedEval::new($opening_score, $ending_score)
    };
}
pub(crate) use s;

#[derive(Copy, Clone, Default)]

pub struct PackedEval {
    pub data: i32,
}

impl PackedEval {
    pub const fn new(opening: i16, ending: i16) -> Self {
        Self { data: ((ending as i32) << 16) + opening as i32 }
    }

    pub const fn new_raw(data: i32) -> Self {
        Self { data }
    }

    pub fn get_opening(&self) -> i16 {
        self.data as i16
    }

    pub fn get_ending(&self) -> i16 {
        ((self.data + 0x8000) >> 16) as i16
    }

    /// Blends `opening_score` and `ending_score` with the ratio passed in `game_phase`. The ratio is a number from 0 to `max_game_phase`, where:
    ///  - `max_game_phase` represents a board with the initial state set (opening phase)
    ///  - 0 represents a board without any piece (ending phase)
    ///  - every value between them represents a board state somewhere in the middle game
    pub fn taper_score(&self, game_phase: u8) -> i16 {
        let opening_score = (self.get_opening() as i32) * (game_phase as i32);
        let ending_score = (self.get_ending() as i32) * ((INITIAL_GAME_PHASE as i32) - (game_phase as i32));

        ((opening_score + ending_score) / (INITIAL_GAME_PHASE as i32)) as i16
    }

    #[cfg(feature = "dev")]
    pub fn to_tuner_params(&self, min: i16, min_init: i16, max_init: i16, max: i16, offset: i16) -> [TunerParameter; 2] {
        [
            TunerParameter::new(self.get_opening() + offset, min, min_init, max_init, max),
            TunerParameter::new(self.get_ending() + offset, min, min_init, max_init, max),
        ]
    }
}

impl ops::Add<PackedEval> for PackedEval {
    type Output = PackedEval;

    fn add(self, rhs: PackedEval) -> PackedEval {
        PackedEval::new_raw(self.data + rhs.data)
    }
}

impl ops::AddAssign<PackedEval> for PackedEval {
    fn add_assign(&mut self, rhs: PackedEval) {
        self.data += rhs.data;
    }
}

impl ops::Sub<PackedEval> for PackedEval {
    type Output = PackedEval;

    fn sub(self, rhs: PackedEval) -> PackedEval {
        PackedEval::new_raw(self.data - rhs.data)
    }
}

impl ops::SubAssign<PackedEval> for PackedEval {
    fn sub_assign(&mut self, rhs: PackedEval) {
        self.data -= rhs.data;
    }
}

impl ops::Mul<PackedEval> for i8 {
    type Output = PackedEval;

    fn mul(self, rhs: PackedEval) -> PackedEval {
        PackedEval::new_raw(self as i32 * rhs.data)
    }
}

impl ops::Mul<PackedEval> for i16 {
    type Output = PackedEval;

    fn mul(self, rhs: PackedEval) -> PackedEval {
        PackedEval::new_raw(self as i32 * rhs.data)
    }
}

impl ops::Mul<PackedEval> for i32 {
    type Output = PackedEval;

    fn mul(self, rhs: PackedEval) -> PackedEval {
        PackedEval::new_raw(self * rhs.data)
    }
}
