use crate::basic_geometry::point::Point;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Light {
    pub(crate) position: Point,
}

impl Light {
    pub(crate) fn new(position: Point) -> Light {
        Light { position }
    }
}
