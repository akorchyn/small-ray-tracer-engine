use std::ops::Add;

use crate::basic_geometry::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Point {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub(crate) fn to_vector(&self) -> Vector {
        Vector::new(self.x, self.y, self.z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
