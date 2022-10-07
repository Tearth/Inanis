pub trait DivCeil {
    type Item;

    fn div_ceil_stable(&self, divider: Self::Item) -> Self::Item;
}

macro_rules! div_ceil_implementation {
    ($type:ident) => {
        impl DivCeil for $type {
            type Item = $type;

            #[inline(always)]
            fn div_ceil_stable(&self, divider: $type) -> Self::Item {
                // Integer ceiling: https://stackoverflow.com/a/2745086
                (self + divider - 1) / divider
            }
        }
    };
}

div_ceil_implementation!(u8);
div_ceil_implementation!(u16);
div_ceil_implementation!(u32);
div_ceil_implementation!(u64);
div_ceil_implementation!(usize);
