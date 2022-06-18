pub(crate) mod alighned_box;
pub(crate) mod disk;
pub(crate) mod normal;
pub(crate) mod plane;
pub(crate) mod point;
pub(crate) mod ray;
pub(crate) mod sphere;
pub(crate) mod vector;

use normal::Normal;
use point::Point;
use ray::Ray;

pub(crate) trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

pub(crate) trait NormalAtPoint {
    fn normal_at_point(&self, point: &Point) -> Normal;
}
