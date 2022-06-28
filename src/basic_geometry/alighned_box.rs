use crate::basic_geometry::vector::Vector;
use crate::basic_geometry::Normal;
use crate::basic_geometry::Point;
use crate::basic_geometry::Ray;
use crate::complex_structures::BoundingBox;

use super::Axis;
use super::Intersect;
use super::Intersection;
use super::NormalAtPoint;
use super::Transform;

#[derive(Debug, Clone, Copy)]
pub(crate) struct AlighnedBox {
    pub(crate) min: Point,
    pub(crate) max: Point,
}

#[allow(dead_code)]
impl AlighnedBox {
    pub(crate) fn new(min: Point, max: Point) -> AlighnedBox {
        AlighnedBox { min, max }
    }

    pub(crate) fn from_dimensions(
        center: Point,
        width: f64,
        height: f64,
        length: f64,
    ) -> AlighnedBox {
        let size_vector = Vector::new(width, height, length);
        let center: Vector = center.into();
        AlighnedBox::new((center - size_vector).into(), (center + size_vector).into())
    }

    pub(crate) fn center(&self) -> Point {
        Point::from((Vector::from(self.min) + Vector::from(self.max)) / 2.)
    }

    pub(crate) fn union(&self, other: &AlighnedBox) -> AlighnedBox {
        AlighnedBox::new(
            Point::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            Point::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        )
    }

    pub(crate) fn union_point(&self, other: Point) -> AlighnedBox {
        AlighnedBox::new(other, other).union(self)
    }

    pub(crate) fn longest_axis(&self) -> Axis {
        let x_axis_length = self.max.x - self.min.x;
        let y_axis_length = self.max.y - self.min.y;
        let z_axis_length = self.max.z - self.min.z;
        if x_axis_length > y_axis_length && x_axis_length > z_axis_length {
            Axis::X
        } else if y_axis_length > z_axis_length {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    pub(crate) fn offset(&self, p: Point) -> Vector {
        let mut o = p - self.min;
        if self.max.x > self.min.x {
            o.x /= self.max.x - self.min.x;
        }
        if self.max.y > self.min.y {
            o.y /= self.max.y - self.min.y;
        }
        if self.max.z > self.min.z {
            o.z /= self.max.z - self.min.z;
        }
        o
    }

    pub(crate) fn surface_area(&self) -> f64 {
        let d = self.max - self.min;
        2. * (d.x * d.y + d.x * d.z + d.y * d.z)
    }
}

impl Intersect for AlighnedBox {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut tmin = (self.min.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.max.x - ray.origin.x) / ray.direction.x;

        let swap_tmin_tmax = |tmin: &mut f64, tmax: &mut f64| {
            if tmin > tmax {
                std::mem::swap(tmin, tmax);
            }
        };
        swap_tmin_tmax(&mut tmin, &mut tmax);

        let mut tymin = (self.min.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.max.y - ray.origin.y) / ray.direction.y;

        swap_tmin_tmax(&mut tymin, &mut tymax);
        if tmin > tymax || tymin > tmax {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.max.z - ray.origin.z) / ray.direction.z;

        swap_tmin_tmax(&mut tzmin, &mut tzmax);
        if tmin > tzmax || tzmin > tmax {
            return None;
        }
        if tzmin > tmin {
            tmin = tzmin;
        }

        if tmin.is_infinite() || tmin.is_nan() {
            None
        } else {
            Some(Intersection::Intersect(tmin))
        }
    }
}

impl NormalAtPoint for AlighnedBox {
    fn normal_at_point(&self, point: &Point, _: Intersection) -> Normal {
        if (point.x - self.min.x).abs() < 0.001 {
            Normal::new(-1., 0., 0.)
        } else if (point.x - self.max.x).abs() < 0.001 {
            Normal::new(1., 0., 0.)
        } else if (point.y - self.min.y).abs() < 0.001 {
            Normal::new(0., -1., 0.)
        } else if (point.y - self.max.y).abs() < 0.001 {
            Normal::new(0., 1., 0.)
        } else if (point.z - self.min.z).abs() < 0.001 {
            Normal::new(0., 0., -1.)
        } else if (point.z - self.max.z).abs() < 0.001 {
            Normal::new(0., 0., 1.)
        } else {
            panic!("point is not in the box {:?}", point);
        }
    }
}

impl Default for AlighnedBox {
    fn default() -> AlighnedBox {
        let smallest = std::f64::MIN;
        let largest = std::f64::MAX;
        AlighnedBox::new(
            Point::new(largest, largest, largest),
            Point::new(smallest, smallest, smallest),
        )
    }
}

impl Transform for AlighnedBox {
    fn transform(&mut self, tranform: super::Transformation) {
        let matrix = tranform.transformation_to_matrix();
        self.min = matrix * self.min;
        self.max = matrix * self.max;
    }
}

impl BoundingBox for AlighnedBox {
    fn bounding_box(&self) -> AlighnedBox {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_test() {
        let box_ = AlighnedBox::from_dimensions(Point::new(0., 0., 0.), 10., 10., 10.);
        let ray = Ray::new(Point::new(0., -20., 0.), Normal::new(0., 1., 0.));
        let t = box_.intersect(&ray);
        assert_eq!(t, Some(Intersection::Intersect(10.)));
    }
}
