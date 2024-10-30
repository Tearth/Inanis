pub trait MinMax {
    fn min() -> Self;
    fn max() -> Self;
}

macro_rules! min_max {
    ($type:ident) => {
        impl MinMax for $type {
            /// Gets a minimal possible value for the type.
            fn min() -> $type {
                $type::MIN
            }

            /// Gets a maximal possible value for the type.
            fn max() -> $type {
                $type::MAX
            }
        }
    };
}

min_max!(i8);
min_max!(u8);
min_max!(i16);
min_max!(u16);
min_max!(i32);
min_max!(u32);
min_max!(i64);
min_max!(u64);
min_max!(isize);
min_max!(usize);
