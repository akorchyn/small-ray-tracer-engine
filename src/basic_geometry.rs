pub(crate) mod alighned_box;
pub(crate) mod disk;
pub(crate) mod matrix;
pub(crate) mod normal;
pub(crate) mod plane;
pub(crate) mod point;
pub(crate) mod ray;
pub(crate) mod sphere;
pub(crate) mod triangle;
pub(crate) mod vector;

use matrix::Matrix;
use normal::Normal;
use point::Point;
use ray::Ray;
use vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Intersection {
    Intersect(f64),
    TriangleIntesersect(f64, f64, f64),
}

impl Intersection {
    pub(crate) fn distance(&self) -> f64 {
        match self {
            &Intersection::Intersect(distance) => distance,
            &Intersection::TriangleIntesersect(distance, _, _) => distance,
        }
    }
}

pub(crate) trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub(crate) trait NormalAtPoint {
    fn normal_at_point(&self, point: &Point, intersection: Intersection) -> Normal;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Transformation {
    Translation(Vector),
    Rotation(Axis, f64),
    Scale(Vector),
}

impl Transformation {
    pub(crate) fn transformation_to_matrix(&self) -> Matrix<4, 4> {
        match *self {
            Transformation::Rotation(axis, angle) => match axis {
                Axis::X => Matrix::<4, 4>::rotation_x(angle.to_radians()),
                Axis::Y => Matrix::<4, 4>::rotation_y(angle.to_radians()),
                Axis::Z => Matrix::<4, 4>::rotation_z(angle.to_radians()),
            },
            Transformation::Translation(vector) => Matrix::<4, 4>::translation(vector),
            Transformation::Scale(vector) => Matrix::<4, 4>::scale(vector),
        }
    }
}

pub(crate) trait Transform {
    fn transform(&mut self, tranform: Transformation);
}
