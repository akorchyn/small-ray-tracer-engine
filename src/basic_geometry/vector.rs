use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use crate::basic_geometry::matrix::Matrix;
use crate::basic_geometry::normal::Normal;
use crate::basic_geometry::point::Point;

use super::Axis;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Vector {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) z: f64,
}

impl Vector {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub(crate) fn dot(&self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub(crate) fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub(crate) fn cross(&self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub(crate) fn normalize(&self) -> Normal {
        let length = self.length();
        if length == 0.0 {
            return Normal::new(0.0, 0.0, 0.0);
        }
        Normal::new(self.x / length, self.y / length, self.z / length)
    }
}

impl Mul for Vector {
    type Output = f64;

    fn mul(self, other: Vector) -> f64 {
        self.dot(other)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl From<Point> for Vector {
    fn from(point: Point) -> Vector {
        Vector::new(point.x, point.y, point.z)
    }
}

impl From<Normal> for Vector {
    fn from(normal: Normal) -> Vector {
        Vector::new(normal.x, normal.y, normal.z)
    }
}

impl From<Matrix<3, 1>> for Vector {
    fn from(matrix: Matrix<3, 1>) -> Vector {
        Vector::new(matrix[0][0], matrix[1][0], matrix[2][0])
    }
}

impl From<Matrix<1, 3>> for Vector {
    fn from(matrix: Matrix<1, 3>) -> Vector {
        Vector::new(matrix[0][0], matrix[0][1], matrix[0][2])
    }
}

impl Index<Axis> for Vector {
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
    use crate::basic_geometry::vector::Vector;

    #[test]
    fn test_new() {
        let vector = Vector::new(0.5, 0.5, 0.5);
        assert_eq!(vector.x, 0.5);
        assert_eq!(vector.y, 0.5);
        assert_eq!(vector.z, 0.5);
    }

    #[test]
    fn test_dot() {
        let vector1 = Vector::new(0.5, 0.5, 0.5);
        let vector2 = Vector::new(0.5, 0.5, 0.5);
        assert_eq!(vector1.dot(vector2), 0.75);
    }

    #[test]
    fn test_vector_length() {
        let vector = Vector::new(3., 4., 0.);
        assert_eq!(vector.length(), 5.);
    }

    #[test]
    fn test_vector_normalization() {
        let vector = Vector::new(3., 4., 0.);
        let normal = vector.normalize();
        assert_eq!(normal.x, 0.6);
        assert_eq!(normal.y, 0.8);
        assert_eq!(normal.z, 0.);
    }

    #[test]
    fn test_vector_add() {
        let vector1 = Vector::new(1., 2., 3.);
        let vector2 = Vector::new(4., 5., 6.);
        let vector3 = vector1 + vector2;
        assert_eq!(vector3.x, 5.);
        assert_eq!(vector3.y, 7.);
        assert_eq!(vector3.z, 9.);
    }

    #[test]
    fn vector_sub() {
        let vector1 = Vector::new(1., 2., 3.);
        let vector2 = Vector::new(4., 5., 6.);
        let vector3 = vector1 - vector2;
        assert_eq!(vector3.x, -3.);
        assert_eq!(vector3.y, -3.);
        assert_eq!(vector3.z, -3.);
    }

    #[test]
    fn vector_neg() {
        let vector = Vector::new(1., 2., 3.);
        let vector2 = -vector;
        assert_eq!(vector2.x, -1.);
        assert_eq!(vector2.y, -2.);
        assert_eq!(vector2.z, -3.);
    }

    #[test]
    fn vector_div_by_value() {
        let vector = Vector::new(1., 2., 3.);
        let vector2 = vector / 2.;
        assert_eq!(vector2.x, 0.5);
        assert_eq!(vector2.y, 1.);
        assert_eq!(vector2.z, 1.5);
    }

    #[test]
    fn cross_product_test() {
        let vector1 = Vector::new(3.0, -5.0, 4.0);
        let vector2 = Vector::new(2.0, 6.0, 5.0);
        let vector3 = vector1.cross(vector2);
        assert_eq!(vector3.x, -49.0);
        assert_eq!(vector3.y, -7.0);
        assert_eq!(vector3.z, 28.0);
    }

    #[test]
    fn cross_product_2() {
        let vector1 = Vector::new(3.0, 5.0, 4.0);
        let vector2 = Vector::new(2.0, 7.0, 5.0);
        let vector3 = vector1.cross(vector2);
        assert_eq!(vector3.x, -3.0);
        assert_eq!(vector3.y, -7.0);
        assert_eq!(vector3.z, 11.0);
    }
}
