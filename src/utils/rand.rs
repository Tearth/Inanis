use std::cell::Cell;
use std::ops::Bound;
use std::ops::RangeBounds;

pub struct RandState {
    pub seed: Cell<u64>,
}

impl RandState {
    /// Constructs a new instance of [RandState] with stored `seed`.
    pub fn new(seed: u64) -> Self {
        Self { seed: Cell::new(seed) }
    }
}

thread_local! {
     static SEED: RandState = RandState::new(common::time::get_unix_timestamp())
}

macro_rules! rand_definition {
    ($type:ident, $min_value:expr, $max_value:expr) => {
        /// Gets a random number within `range`.
        pub fn $type(range: impl RangeBounds<$type>) -> $type {
            let from = match range.start_bound() {
                Bound::Included(v) => *v,
                Bound::Excluded(v) => *v + 1,
                Bound::Unbounded => $min_value,
            };

            let to = match range.end_bound() {
                Bound::Included(v) => *v,
                Bound::Excluded(v) => *v - 1,
                Bound::Unbounded => $max_value,
            };

            if from == $min_value && to == $max_value {
                rand_internal() as $type
            } else {
                (rand_internal() % (((to as i128) - (from as i128) + 1) as u64)) as $type + from
            }
        }
    };
}

rand_definition!(i8, i8::MIN, i8::MAX);
rand_definition!(u8, u8::MIN, u8::MAX);
rand_definition!(i16, i16::MIN, i16::MAX);
rand_definition!(u16, u16::MIN, u16::MAX);
rand_definition!(i32, i32::MIN, i32::MAX);
rand_definition!(u32, u32::MIN, u32::MAX);
rand_definition!(i64, i64::MIN, i64::MAX);
rand_definition!(u64, u64::MIN, u64::MAX);
rand_definition!(isize, isize::MIN, isize::MAX);
rand_definition!(usize, usize::MIN, usize::MAX);

/// Sets an initial seed for LCG.
pub fn seed(seed: u64) {
    SEED.with(|state| {
        state.seed.set(seed);
    });
}

fn rand_internal() -> u64 {
    // https://en.wikipedia.org/wiki/Xorshift#xorshift*
    SEED.with(|state| {
        let mut x = state.seed.get();
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        state.seed.set(x);

        x.wrapping_mul(0x2545f4914f6cdd1d)
    })
}
