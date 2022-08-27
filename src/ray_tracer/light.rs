use crate::basic_geometry::{normal::Normal, point::Point};

use super::color::Color;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Light {
    Environment(Color, f64),
    Point(Point, Color, f64),
    Directed(Normal, Color, f64),
}
