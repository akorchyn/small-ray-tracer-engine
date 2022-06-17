use crate::basic_geometry::normal::Normal;
use crate::basic_geometry::point::Point;
use crate::basic_geometry::ray::Ray;

use super::Intersect;

pub(crate) struct Plane {
    pub(crate) normal: Normal,
    pub(crate) center: Point,
}

impl Plane {
    pub(crate) fn new(normal: Normal, center: Point) -> Plane {
        Plane { normal, center }
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // (p - c)*n = 0 plane equation
        // p = (o + t*d) ray equation
        // k = (o - c)
        // (t*d + k)*n = 0;
        // t*d*n = -k*n;
        // t = -k*n / d*n;
        // if d * n == 0 then the ray is parallel to the plane, so no intersection
        // if t < 0 then the ray is pointing away from the plane, so no intersection

        let normal = self.normal.to_vector();
        let dn = ray.direction.to_vector().dot(normal);

        if dn.abs() <= f64::EPSILON {
            return None;
        }

        let k = self.center.to_vector() - ray.origin.to_vector();
        let t = k.dot(normal) / dn;
        if t >= 0. {
            Some(t)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::basic_geometry::point::Point;

    #[test]
    fn intersection_test() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(0., 1., 0.));
        let plane = Plane::new(Normal::new(0., -1., 0.), Point::new(0., 50., 0.));

        assert_eq!(plane.intersect(&ray), Some(50.));
    }

    #[test]
    fn intersection_failure_test() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(1., 0., 0.));
        let plane = Plane::new(Normal::new(0., -1., 0.), Point::new(0., 50., 0.));

        assert_eq!(plane.intersect(&ray), None);
    }

    #[test]
    fn ray_intersect_behind() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(0., -1., 0.));
        let plane = Plane::new(Normal::new(0., -1., 0.), Point::new(0., 50., 0.));

        assert_eq!(plane.intersect(&ray), None);
    }

    #[test]
    fn intersection_shift_ok() {
        let ray = Ray::new(Point::new(2., 0., 0.), Normal::new(0., 1., 0.));
        let plane = Plane::new(Normal::new(0., 1., 0.), Point::new(0., 10., 0.));

        assert_eq!(plane.intersect(&ray), Some(10.));
    }

    #[test]
    fn paralel_ray_to_plane() {
        let ray = Ray::new(Point::new(1., 0., 0.), Normal::new(1., 0., 0.));
        let plane = Plane::new(Normal::new(0., 1., 0.), Point::new(2., 0., 0.));

        assert_eq!(plane.intersect(&ray), None);
    }
}
