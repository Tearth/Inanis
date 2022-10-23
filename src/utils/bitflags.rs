pub trait BitFlags {
    type Item;

    fn contains(&self, value: Self::Item) -> bool;
}

macro_rules! bit_flags_implementation {
    ($type:ident) => {
        impl BitFlags for $type {
            type Item = $type;

            /// Checks if the specified flags (bytes) are present.
            #[inline(always)]
            fn contains(&self, value: $type) -> bool {
                (self & value) != 0
            }
        }
    };
}

bit_flags_implementation!(i8);
bit_flags_implementation!(u8);
bit_flags_implementation!(i16);
bit_flags_implementation!(u16);
bit_flags_implementation!(i32);
bit_flags_implementation!(u32);
bit_flags_implementation!(i64);
bit_flags_implementation!(u64);
bit_flags_implementation!(isize);
bit_flags_implementation!(usize);
