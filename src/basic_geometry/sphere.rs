use super::alighned_box::AlighnedBox;
use super::{Intersect, Intersection, NormalAtPoint, Transform, Transformation};
use crate::basic_geometry::normal::Normal;
use crate::basic_geometry::point::Point;
use crate::basic_geometry::ray::Ray;
use crate::basic_geometry::vector::Vector;
use crate::complex_structures::BoundingBox;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    #[allow(dead_code)]
    pub(crate) fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let k = Vector::from(ray.origin) - Vector::from(self.center);
        let a = ray.direction.dot(ray.direction);
        let b = 2. * k.dot(Vector::from(ray.direction));
        let c = k.dot(k) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return None;
        }
        let square_descriminant = discriminant.sqrt();
        let t1 = (-b - square_descriminant) / (2. * a);
        let t2 = (-b + square_descriminant) / (2. * a);

        if t1 > 0. && t2 > 0. {
            Some(Intersection::Intersect(t1.min(t2)))
        } else if t1 > 0. {
            Some(Intersection::Intersect(t1))
        } else if t2 > 0. {
            Some(Intersection::Intersect(t2))
        } else {
            None
        }
    }
}

impl NormalAtPoint for Sphere {
    fn normal_at_point(&self, point: &Point, _: Intersection) -> Normal {
        (Vector::from(*point) - Vector::from(self.center)).normalize()
    }
}

impl Transform for Sphere {
    fn transform(&mut self, transformation: Transformation) {
        match transformation {
            Transformation::Translation(_) => {
                self.center = transformation.transformation_to_matrix() * self.center;
            }
            Transformation::Scale(scale) => {
                self.center = transformation.transformation_to_matrix() * self.center;
                // TODO: if scale is not uniform, this will not work, because it suppose to become a ellipsoid
                self.radius *= scale.x.max(scale.y).max(scale.z);
            }
            Transformation::Rotation(_, _) => {}
        }
    }
}

impl BoundingBox for Sphere {
    fn bounding_box(&self) -> AlighnedBox {
        let point = Point::new(self.radius, self.radius, self.radius);
        AlighnedBox::new(
            Point::new(
                self.center.x - self.radius,
                self.center.y - self.radius,
                self.center.z - self.radius,
            ),
            Point::new(
                self.center.x + self.radius,
                self.center.y + self.radius,
                self.center.z + self.radius,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::basic_geometry::normal::Normal;

    use super::*;

    #[test]
    fn intersection_test() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(0., 1., 0.));
        let sphere = Sphere {
            center: Point::new(0., 5., 0.),
            radius: 1.,
        };

        assert_eq!(sphere.intersect(&ray), Some(Intersection::Intersect(4.)));
    }

    #[test]
    fn intersection_fail_test() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(1., 0., 0.));
        let sphere = Sphere {
            center: Point::new(0., 5., 0.),
            radius: 0.5,
        };

        assert_eq!(sphere.intersect(&ray), None);
    }

    #[test]
    fn intersection_behind_test() {
        let ray = Ray::new(Point::new(0., 0., 0.), Normal::new(0., -1., 0.));
        let sphere = Sphere {
            center: Point::new(0., 5., 0.),
            radius: 1.,
        };

        assert_eq!(sphere.intersect(&ray), None);
    }
}
