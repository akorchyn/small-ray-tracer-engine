use std::ops::{Add, Mul, Neg, Sub};

use crate::basic_geometry::normal::Normal;

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
}
