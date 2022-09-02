use std::ops::{Add, Mul, Range};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Bounded<T: PartialOrd + Copy> {
    val: T,
}

impl<T: PartialOrd + Copy> Bounded<T> {
    pub(crate) const fn new(val: T) -> Self {
        Bounded { val }
    }

    pub(crate) fn get_saturated(&self, min: T, max: T) -> T {
        Bounded::saturate(self.val, min, max)
    }

    fn saturate(val: T, min: T, max: T) -> T {
        if min < val && val < max {
            val
        } else if val >= max {
            max
        } else {
            min
        }
    }
}

impl<T: Add<Output = T> + PartialOrd + Copy> Add for Bounded<T> {
    type Output = Bounded<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Bounded::<T>::new(self.val + rhs.val)
    }
}

impl<T: Mul<Output = T> + PartialOrd + Copy> Mul for Bounded<T> {
    type Output = Bounded<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Bounded::<T>::new(self.val * rhs.val)
    }
}
