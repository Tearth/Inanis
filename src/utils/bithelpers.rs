pub trait BitHelpers {
    type Item;

    fn get_lsb(&self) -> Self::Item;
    fn pop_lsb(&self) -> Self::Item;
    fn bit_count(&self) -> usize;
    fn bit_scan(&self) -> usize;
}

macro_rules! bit_helpers {
    ($type:ident) => {
        impl BitHelpers for $type {
            type Item = $type;

            /// Extracts the lowest set isolated bit.
            ///
            /// More about asm instruction: <https://www.felixcloutier.com/x86/blsi>
            fn get_lsb(&self) -> Self::Item {
                self & self.wrapping_neg()
            }

            /// Resets the lowest set bit.
            ///
            /// More about asm instruction: <https://www.felixcloutier.com/x86/blsr>
            fn pop_lsb(&self) -> Self::Item {
                self & (self - 1)
            }

            /// Counts the number of set bits.
            ///
            /// More about asm instruction: <https://www.felixcloutier.com/x86/popcnt>
            fn bit_count(&self) -> usize {
                self.count_ones() as usize
            }

            /// Gets an index of the first set bit by counting trailing zero bits.
            ///
            /// More about asm instruction: <https://www.felixcloutier.com/x86/tzcnt>
            fn bit_scan(&self) -> usize {
                self.trailing_zeros() as usize
            }
        }
    };
}

bit_helpers!(i8);
bit_helpers!(u8);
bit_helpers!(i16);
bit_helpers!(u16);
bit_helpers!(i32);
bit_helpers!(u32);
bit_helpers!(i64);
bit_helpers!(u64);
bit_helpers!(isize);
bit_helpers!(usize);
