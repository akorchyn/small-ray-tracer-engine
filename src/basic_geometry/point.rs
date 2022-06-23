use std::ops::{Add, Index, Sub};

use crate::basic_geometry::vector::Vector;

use super::{matrix::Matrix, Axis};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
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

impl From<Matrix<3, 1>> for Point {
    fn from(matrix: Matrix<3, 1>) -> Point {
        Point::new(matrix[0][0], matrix[1][0], matrix[2][0])
    }
}

impl From<Matrix<1, 3>> for Point {
    fn from(matrix: Matrix<1, 3>) -> Point {
        Point::new(matrix[0][0], matrix[0][1], matrix[0][2])
    }
}

impl Index<Axis> for Point {
    type Output = f64;
    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
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
