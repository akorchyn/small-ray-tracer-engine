use std::{cell::RefCell, rc::Rc};

use crate::{
    basic_geometry::{Intersect, NormalAtPoint},
    complex_structures::BoundingBox,
};

use super::{texture::Texture, RayTracable};

#[derive(Clone)]
pub(crate) struct Object {
    geometry: Rc<RefCell<dyn RayTracable>>,
    pub(crate) texture: Texture,
}

impl Object {
    pub(crate) fn lambert(geometry: Rc<RefCell<dyn RayTracable>>) -> Self {
        Self {
            geometry,
            texture: Texture::Lambert,
        }
    }

    pub(crate) fn reflection(geometry: Rc<RefCell<dyn RayTracable>>, coof: f64) -> Self {
        assert!(0.0 <= coof && coof <= 1.);
        Self {
            geometry,
            texture: Texture::Reflection(coof),
        }
    }
}

impl Intersect for Object {
    fn intersect(
        &self,
        ray: &crate::basic_geometry::ray::Ray,
    ) -> Option<crate::basic_geometry::Intersection> {
        self.geometry.borrow().intersect(ray)
    }
}

impl NormalAtPoint for Object {
    fn normal_at_point(
        &self,
        point: &crate::basic_geometry::point::Point,
        intersection: crate::basic_geometry::Intersection,
    ) -> crate::basic_geometry::normal::Normal {
        self.geometry.borrow().normal_at_point(point, intersection)
    }
}

impl BoundingBox for Object {
    fn bounding_box(&self) -> crate::basic_geometry::alighned_box::AlighnedBox {
        self.geometry.borrow().bounding_box()
    }
}
