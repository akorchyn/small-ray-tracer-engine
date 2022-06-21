use crate::basic_geometry::normal::Normal;
use crate::basic_geometry::point::Point;
use crate::basic_geometry::ray::Ray;
use crate::basic_geometry::vector::Vector;

use super::plane::Plane;
use super::{Intersect, Intersection};

pub(crate) struct Disk {
    center: Point,
    radius: f64,
    normal: Normal,
}

impl Disk {
    #[allow(dead_code)]
    pub(crate) fn new(center: Point, radius: f64, normal: Normal) -> Disk {
        Disk {
            center,
            radius,
            normal,
        }
    }
}

impl Intersect for Disk {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let plane = Plane::new(self.normal, self.center);
        match plane.intersect(ray) {
            Intersection::Intersect(t) if t > 0. => {
                let point = ray.at(t);
                let distance = (Vector::from(point) - Vector::from(self.center)).length();
                if distance < self.radius {
                    Intersection::Intersect(t)
                } else {
                    Intersection::DoesNotIntersect
                }
            }
            _ => Intersection::DoesNotIntersect,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_test() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(0., 1., 0.));
        let disk = Disk::new(Point::new(0., 10., 0.), 1., Normal::new(0., 1., 0.));

        assert_eq!(disk.intersect(&ray), Intersection::Intersect(10.));
    }

    #[test]
    fn intersection_failure_test() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(1., 0., 0.));
        let disk = Disk::new(Point::new(0., 10., 0.), 1., Normal::new(0., 1., 0.));

        assert_eq!(disk.intersect(&ray), Intersection::DoesNotIntersect);
    }

    #[test]
    fn intersection_behind() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(0., -1., 0.));
        let disk = Disk::new(Point::new(0., 10., 0.), 1., Normal::new(0., 1., 0.));

        assert_eq!(disk.intersect(&ray), Intersection::DoesNotIntersect);
    }

    #[test]
    fn insersection_corner_failure() {
        let ray = Ray::new(Point::new(2., 0., 0.), Normal::new(0., 1., 0.));
        let disk = Disk::new(Point::new(0., 10., 0.), 1., Normal::new(0., 1., 0.));

        assert_eq!(disk.intersect(&ray), Intersection::DoesNotIntersect);
    }
}
