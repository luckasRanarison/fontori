use std::ops::{BitAnd, Shr};

pub trait BitFlag<F> {
    fn get(self, flag: F) -> F;
    fn has(self, flag: F) -> bool;
}

impl<T> BitFlag<T> for T
where
    T: Clone + Copy + PartialEq + Shr<T, Output = T> + BitAnd<T, Output = T> + From<u8>,
{
    fn get(self, flag: T) -> T {
        self >> flag & T::from(1)
    }

    fn has(self, flag: T) -> bool {
        self.get(flag) == T::from(1)
    }
}
