use std::ops::Mul;

use crate::basic_geometry::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Normal {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Normal {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Normal {
        debug_assert!(
            x <= 1. && y <= 1. && z <= 1.,
            "Normal must be between 0 and 1, but have {:?}",
            (x, y, z)
        );
        Normal { x, y, z }
    }

    pub(crate) fn dot(&self, other: Normal) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Normal {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::basic_geometry::normal::Normal;
    use crate::basic_geometry::vector::Vector;

    #[test]
    fn test_new() {
        let normal = Normal::new(0.5, 0.5, 0.5);
        assert_eq!(normal.x, 0.5);
        assert_eq!(normal.y, 0.5);
        assert_eq!(normal.z, 0.5);
    }

    #[test]
    fn test_dot() {
        let normal1 = Normal::new(0.5, 0.5, 0.5);
        let normal2 = Normal::new(0.5, 0.5, 0.5);
        assert_eq!(normal1.dot(normal2), 0.75);
    }

    #[test]
    fn test_to_vector() {
        let normal = Normal::new(0.5, 0.5, 0.5);
        let vector: Vector = normal.into();
        assert_eq!(vector.x, 0.5);
        assert_eq!(vector.y, 0.5);
        assert_eq!(vector.z, 0.5);
    }

    #[test]
    #[should_panic]
    fn normal_panicking_if_value_more_than_1() {
        let _ = Normal::new(1.5, 0.5, 0.5);
    }
}
