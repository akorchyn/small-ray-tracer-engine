use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

use crate::basic_types::bounded::Bounded;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct Color {
    r: Bounded<f64>,
    g: Bounded<f64>,
    b: Bounded<f64>,
}

impl Color {
    pub(crate) const fn bounded_new(r: Bounded<f64>, g: Bounded<f64>, b: Bounded<f64>) -> Self {
        Color { r, g, b }
    }

    pub(crate) const fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            r: Bounded::new(r),
            g: Bounded::new(g),
            b: Bounded::new(b),
        }
    }

    pub(crate) const fn black() -> Self {
        Color::new(0., 0., 0.)
    }

    pub(crate) const fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }

    pub(crate) const fn blue() -> Self {
        Color::new(0., 0., 1.)
    }

    pub(crate) const fn red() -> Self {
        Color::new(1., 0., 0.)
    }

    pub(crate) fn rgb(&self) -> [u8; 3] {
        [
            (self.r.get_saturated(0., 1.) * 255.) as u8,
            (self.g.get_saturated(0., 1.) * 255.) as u8,
            (self.b.get_saturated(0., 1.) * 255.) as u8,
        ]
    }
}

impl From<[f32; 3]> for Color {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Color::new(r.into(), g.into(), b.into())
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::bounded_new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, coof: f64) -> Self::Output {
        let coof = Bounded::new(coof);
        Color::bounded_new(self.r * coof, self.g * coof, self.b * coof)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, coof: Color) -> Self::Output {
        Color::bounded_new(self.r * coof.r, self.g * coof.g, self.b * coof.b)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::bounded_new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::black(), |a, b| a + b)
    }
}
