use crate::basic_geometry::point::Point;

use super::{
    normal::Normal, ray::Ray, vector::Vector, Intersect, Intersection, NormalAtPoint, Transform,
    Transformation,
};

pub(crate) struct Triangle {
    a: Vector,
    b: Vector,
    c: Vector,
    na: Normal,
    nb: Normal,
    nc: Normal,
    normal_at_point: bool,
}

impl Triangle {
    pub(crate) fn new(a: Point, b: Point, c: Point) -> Triangle {
        let a = Vector::from(a);
        let b = Vector::from(b);
        let c = Vector::from(c);
        let ab = b - a;
        let ac = c - a;
        let n = ab.cross(ac).normalize();

        Triangle {
            a,
            b,
            c,
            na: n,
            nb: n,
            nc: n,
            normal_at_point: false,
        }
    }
    pub(crate) fn with_normals(
        a: Point,
        na: Normal,
        b: Point,
        nb: Normal,
        c: Point,
        nc: Normal,
    ) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
            na,
            nb,
            nc,
            normal_at_point: true,
        }
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let normal = Vector::from(ray.direction).cross(ac);

        let d = ab.dot(normal);
        if d.abs() < f64::EPSILON {
            return Intersection::DoesNotIntersect;
        }

        let inv_d = 1.0 / d;
        let ao = Vector::from(ray.origin) - self.a;
        let u_coordinate = ao.dot(normal) * inv_d;
        if !(0.0..=1.0).contains(&u_coordinate) {
            return Intersection::DoesNotIntersect;
        }

        let ao_cross_ab = ao.cross(ab);
        let v_coordinate = Vector::from(ray.direction).dot(ao_cross_ab) * inv_d;
        if v_coordinate < 0.0 || u_coordinate + v_coordinate > 1.0 {
            return Intersection::DoesNotIntersect;
        }
        let t = ac.dot(ao_cross_ab) * inv_d;
        if t > f64::EPSILON {
            Intersection::TriangleIntesersect(t, u_coordinate, v_coordinate)
        } else {
            Intersection::DoesNotIntersect
        }
    }
}

impl NormalAtPoint for Triangle {
    fn normal_at_point(&self, _: &Point, intersection: Intersection) -> Normal {
        match intersection {
            Intersection::TriangleIntesersect(_, u, v) => {
                if self.normal_at_point {
                    (self.na * u + self.nb * v + self.nc * (1.0 - u - v)).normalize()
                } else {
                    self.na
                }
            }
            _ => panic!("Called with wrong intersaction type"),
        }
    }
}

impl Transform for Triangle {
    fn transform(&mut self, transform: Transformation) {
        let matrix = transform.transformation_to_matrix();
        self.a = matrix * self.a;
        self.b = matrix * self.b;
        self.c = matrix * self.c;
        self.na = matrix * self.na;
        self.nb = matrix * self.nb;
        self.nc = matrix * self.nc;
    }
}

#[cfg(test)]
mod tests {
    use crate::basic_geometry::normal::Normal;
    use crate::basic_geometry::point::Point;
    use crate::basic_geometry::ray::Ray;
    use crate::basic_geometry::triangle::Triangle;
    use crate::basic_geometry::*;

    #[test]
    fn test_intersection() {
        let triangle = Triangle::new(
            Point::new(0., 0., 0.),
            Point::new(1., 0., 0.),
            Point::new(0., 1., 0.),
        );
        let ray = Ray::new(Point::new(0.5, 0.5, 2.0), Normal::new(0., 0.0, -0.5));
        assert_eq!(
            triangle.intersect(&ray),
            Intersection::TriangleIntesersect(4.0, 0.5, 0.5)
        );
    }

    #[test]
    fn no_intersection() {
        let triangle = Triangle::new(
            Point::new(0., 0., 0.),
            Point::new(1., 0., 0.),
            Point::new(0., 1., 0.),
        );
        let ray = Ray::new(Point::new(1.1, 0.5, 2.0), Normal::new(0., 0.0, -0.5));
        assert_eq!(triangle.intersect(&ray), Intersection::DoesNotIntersect);
    }
}
