use super::color::Color;

pub(crate) struct Material {
    pub(crate) ambient: Color,
    pub(crate) diffuse: Color,
    pub(crate) specular: Color,
    pub(crate) shininess: f64,
}

impl Material {
    pub(crate) fn lambert() -> Self {
        Material {
            ambient: [0.2, 0.2, 0.2].into(),
            diffuse: [0.8, 0.8, 0.8].into(),
            specular: [0.4, 0.4, 0.4].into(),
            shininess: 10.,
        }
    }
}

impl From<tobj::Material> for Material {
    fn from(mat: tobj::Material) -> Self {
        Material {
            ambient: mat.ambient.into(),
            diffuse: mat.diffuse.into(),
            specular: mat.specular.into(),
            shininess: mat.shininess.into(),
        }
    }
}
