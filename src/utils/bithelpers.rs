pub trait BitHelpers {
    type Item;

    fn get_lsb(&self) -> Self::Item;
    fn pop_lsb(&self) -> Self::Item;
    fn bit_count(&self) -> u8;
    fn bit_scan(&self) -> u8;
}

macro_rules! bit_helpers_implementation {
    ($type:ident) => {
        impl BitHelpers for $type {
            type Item = $type;

            /// Extracts the lowest set isolated bit.
            ///
            /// More about asm instruction: <https://www.felixcloutier.com/x86/blsi>
            #[inline(always)]
            fn get_lsb(&self) -> Self::Item {
                self & self.wrapping_neg()
            }

            /// Resets the lowest set bit.
            ///
            /// More about asm instruction: <https://www.felixcloutier.com/x86/blsr>
            #[inline(always)]
            fn pop_lsb(&self) -> Self::Item {
                self & (self - 1)
            }

            /// Counts the number of set bits.
            ///
            /// More about asm instruction: <https://www.felixcloutier.com/x86/popcnt>
            #[inline(always)]
            fn bit_count(&self) -> u8 {
                self.count_ones() as u8
            }

            /// Gets an index of the first set bit by counting trailing zero bits.
            ///
            /// More about asm instruction: <https://www.felixcloutier.com/x86/tzcnt>
            #[inline(always)]
            fn bit_scan(&self) -> u8 {
                self.trailing_zeros() as u8
            }
        }
    };
}

bit_helpers_implementation!(i8);
bit_helpers_implementation!(u8);
bit_helpers_implementation!(i16);
bit_helpers_implementation!(u16);
bit_helpers_implementation!(i32);
bit_helpers_implementation!(u32);
bit_helpers_implementation!(i64);
bit_helpers_implementation!(u64);
bit_helpers_implementation!(isize);
bit_helpers_implementation!(usize);
