use std::ops::{Add, Sub};

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
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl From<Vector> for Point {
    fn from(vector: Vector) -> Point {
        Point::new(vector.x, vector.y, vector.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::basic_geometry::point::Point;

    #[test]
    fn vector_from_substraction() {
        let vector = Point::new(4., 5., 6.) - Point::new(1., 2., 3.);
        assert_eq!(vector.x, 3.);
        assert_eq!(vector.y, 3.);
        assert_eq!(vector.z, 3.);
    }
}
