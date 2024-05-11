pub trait BitFlags {
    type Item;

    fn contains(&self, value: Self::Item) -> bool;
}

macro_rules! bit_flags {
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

bit_flags!(i8);
bit_flags!(u8);
bit_flags!(i16);
bit_flags!(u16);
bit_flags!(i32);
bit_flags!(u32);
bit_flags!(i64);
bit_flags!(u64);
bit_flags!(isize);
bit_flags!(usize);
