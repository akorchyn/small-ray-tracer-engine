use crate::basic_geometry::point::Point;
use crate::basic_geometry::ray::Ray;
use crate::basic_geometry::vector::Vector;
use crate::basic_geometry::{Axis, Transform, Transformation};
use crate::ray_tracer::viewframe::ViewFrame;

// Ray-tracing camera.
pub(crate) struct Camera {
    // Camera position.
    position: Point,
    // Camera view frame.
    view_frame: ViewFrame,
    rotation_angles: Vector,
}

impl Camera {
    pub(crate) fn new(position: Point, view_frame: ViewFrame) -> Camera {
        Camera {
            position,
            view_frame,
            rotation_angles: Vector::new(0.0, 0.0, 0.0),
        }
    }

    pub(crate) fn rotation_vector(&self) -> Vector {
        self.rotation_angles
    }

    pub(crate) fn ray_for_pixel(
        &self,
        x: usize,
        y: usize,
        image_width: usize,
        image_height: usize,
    ) -> Ray {
        let point = self
            .view_frame
            .point_on_pixel(x, y, image_width, image_height);
        self.rotate_ray(self.position, point - self.position)
    }

    fn rotate_ray(&self, position: Point, direction: Vector) -> Ray {
        let x =
            Transformation::Rotation(Axis::X, self.rotation_angles.x).transformation_to_matrix();
        let y =
            Transformation::Rotation(Axis::Y, self.rotation_angles.y).transformation_to_matrix();
        let z =
            Transformation::Rotation(Axis::Z, self.rotation_angles.z).transformation_to_matrix();
        let direction = (z * (y * (x * direction))).normalize();
        Ray::new(position, direction)
    }
}

impl Transform for Camera {
    fn transform(&mut self, transform: Transformation) {
        match transform {
            Transformation::Rotation(axis, angle) => match axis {
                Axis::X => self.rotation_angles.x += angle,
                Axis::Y => self.rotation_angles.y += angle,
                Axis::Z => self.rotation_angles.z += angle,
            },
            _ => {
                let matrix = transform.transformation_to_matrix();
                self.position = matrix * self.position;
                self.view_frame.transform(transform);
            }
        }
    }
}
