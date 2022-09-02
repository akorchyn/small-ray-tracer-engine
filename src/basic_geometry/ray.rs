use crate::basic_geometry::normal::Normal;
use crate::basic_geometry::point::Point;

#[derive(Debug)]
pub(crate) struct Ray {
    pub(crate) origin: Point,
    pub(crate) direction: Normal,
}

impl Ray {
    pub(crate) fn new(origin: Point, direction: Normal) -> Ray {
        Ray { origin, direction }
    }

    pub(crate) fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub(crate) fn reflect_from_normal(&self, point: Point, normal: Normal) -> Self {
        let dir = Normal::reflect(normal, self.direction);
        let ray = Ray::new(point, dir);
        Ray::new(ray.at(1e-4), ray.direction)
    }
}
