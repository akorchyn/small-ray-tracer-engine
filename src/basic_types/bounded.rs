use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Bounded<T: PartialOrd + Copy + PartialEq> {
    val: T,
}

impl<T: PartialOrd + Copy + PartialOrd + PartialEq> Bounded<T> {
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

impl<T: Add<Output = T> + PartialOrd + Copy + PartialEq> Add for Bounded<T> {
    type Output = Bounded<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Bounded::<T>::new(self.val + rhs.val)
    }
}

impl<T: Mul<Output = T> + PartialOrd + Copy + PartialEq> Mul for Bounded<T> {
    type Output = Bounded<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Bounded::<T>::new(self.val * rhs.val)
    }
}

impl<T: Sub<Output = T> + PartialOrd + Copy + PartialEq> Sub for Bounded<T> {
    type Output = Bounded<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Bounded::<T>::new(self.val - rhs.val)
    }
}
