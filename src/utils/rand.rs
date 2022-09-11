use std::cell::Cell;
use std::ops::RangeBounds;

struct RandState {
    pub seed: Cell<u64>,
}

thread_local! {
     static SEED: RandState = RandState { seed: Cell::new(common::time::get_unix_timestamp()) }
}

macro_rules! rand_definition {
    ($type:ident, $min_value:expr, $max_value:expr) => {
        pub fn $type(range: impl RangeBounds<$type>) -> $type {
            let from = match range.start_bound() {
                std::ops::Bound::Included(v) => *v,
                std::ops::Bound::Excluded(v) => *v - 1,
                std::ops::Bound::Unbounded => $min_value,
            };

            let to = match range.end_bound() {
                std::ops::Bound::Included(v) => *v,
                std::ops::Bound::Excluded(v) => *v - 1,
                std::ops::Bound::Unbounded => $max_value,
            };

            if from == $min_value && to == $max_value {
                rand_internal() as $type
            } else {
                ((rand_internal() % ((to as u64) - (from as u64) + 1)) + (from as u64)) as $type
            }
        }
    };
}

pub fn seed(seed: u64) {
    SEED.with(|state| {
        state.seed.set(seed);
    });
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

fn rand_internal() -> u64 {
    SEED.with(|state| {
        let mut x = state.seed.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;

        state.seed.set(x);
        x
    })
}