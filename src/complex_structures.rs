use crate::basic_geometry::alighned_box::AlighnedBox;

pub(crate) mod bvh;

pub(crate) trait BoundingBox {
    fn bounding_box(&self) -> AlighnedBox;
}
