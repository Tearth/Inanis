pub trait DivCeil {
    type Item;

    fn div_ceil_stable(&self, divider: Self::Item) -> Self::Item;
}

macro_rules! div_ceil {
    ($type:ident) => {
        impl DivCeil for $type {
            type Item = $type;

            /// Stable implementation of div_ceil.
            #[inline(always)]
            fn div_ceil_stable(&self, divider: $type) -> Self::Item {
                // Integer ceiling: https://stackoverflow.com/a/2745086
                (self + divider - 1) / divider
            }
        }
    };
}

div_ceil!(u8);
div_ceil!(u16);
div_ceil!(u32);
div_ceil!(u64);
div_ceil!(usize);
