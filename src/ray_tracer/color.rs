use std::{
    iter::Sum,
    ops::{Add, Mul},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Color {
    pub(crate) const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub(crate) fn black() -> Self {
        Color::new(0, 0, 0)
    }

    pub(crate) fn white() -> Self {
        Color::new(255, 255, 255)
    }

    pub(crate) fn blue() -> Self {
        Color::new(30, 144, 255)
    }

    pub(crate) fn red() -> Self {
        Color::new(254, 32, 32)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            self.r.wrapping_add(rhs.r),
            self.g.wrapping_add(rhs.g),
            self.b.wrapping_add(rhs.b),
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, coof: f64) -> Self::Output {
        Color::new(
            ((self.r as f64) * coof) as u8,
            ((self.g as f64) * coof) as u8,
            ((self.b as f64) * coof) as u8,
        )
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::black(), |a, b| a + b)
    }
}
