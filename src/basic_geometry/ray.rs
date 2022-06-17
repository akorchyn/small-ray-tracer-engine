use crate::basic_geometry::normal::Normal;
use crate::basic_geometry::point::Point;

#[derive(Debug)]
pub(crate) struct Ray {
    pub(crate) origin: Point,
    pub(crate) direction: Normal,
}

impl Ray {
    pub(crate) fn new(origin: Point, direction: Normal) -> Ray {
        Ray { origin, direction }
    }

    pub(crate) fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
