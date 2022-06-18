use crate::basic_geometry::normal::Normal;
use crate::basic_geometry::point::Point;
use crate::basic_geometry::ray::Ray;
use crate::basic_geometry::vector::Vector;

use super::plane::Plane;
use super::Intersect;

pub(crate) struct Disk {
    center: Point,
    radius: f64,
    normal: Normal,
}

impl Disk {
    pub(crate) fn new(center: Point, radius: f64, normal: Normal) -> Disk {
        Disk {
            center,
            radius,
            normal,
        }
    }
}

impl Intersect for Disk {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let plane = Plane::new(self.normal, self.center);
        match dbg!(plane.intersect(ray)) {
            Some(t) if t >= 0. => {
                let point = ray.at(t);
                let distance = (Vector::from(point) - Vector::from(self.center)).length();
                if distance < self.radius {
                    Some(t)
                } else {
                    None
                }
            }
            _ => None,
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

        assert_eq!(disk.intersect(&ray), Some(10.));
    }

    #[test]
    fn intersection_failure_test() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(1., 0., 0.));
        let disk = Disk::new(Point::new(0., 10., 0.), 1., Normal::new(0., 1., 0.));

        assert_eq!(disk.intersect(&ray), None);
    }

    #[test]
    fn intersection_behind() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(0., -1., 0.));
        let disk = Disk::new(Point::new(0., 10., 0.), 1., Normal::new(0., 1., 0.));

        assert_eq!(disk.intersect(&ray), None);
    }

    #[test]
    fn insersection_corner_failure() {
        let ray = Ray::new(Point::new(2., 0., 0.), Normal::new(0., 1., 0.));
        let disk = Disk::new(Point::new(0., 10., 0.), 1., Normal::new(0., 1., 0.));

        assert_eq!(disk.intersect(&ray), None);
    }
}
