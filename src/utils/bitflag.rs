use std::ops::{BitAnd, Shr};

pub trait BitFlag<F> {
    fn has(self, flag: F) -> bool;
}

impl<T> BitFlag<u8> for T
where
    T: Clone + Copy + PartialEq + Shr<T, Output = T> + BitAnd<T, Output = T> + From<u8>,
{
    fn has(self, flag: u8) -> bool {
        (self >> T::from(flag) & T::from(1)) == T::from(1)
    }
}
