use crate::basic_geometry::normal::Normal;

#[derive(Debug, Clone, Copy)]
pub(crate) struct DirectedLight {
    direction: Normal,
}

impl DirectedLight {
    pub(crate) fn new(direction: Normal) -> DirectedLight {
        DirectedLight { direction }
    }

    pub(crate) fn intensity_at_normal(&self, normal: &Normal) -> f64 {
        let cos_angle = self.direction.dot(*normal);
        if cos_angle < 0.0 {
            0.0
        } else {
            cos_angle
        }
    }
}
