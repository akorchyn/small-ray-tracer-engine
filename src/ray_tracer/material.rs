use super::color::Color;

pub(crate) struct Material {
    pub(crate) ambient: Color,
    pub(crate) diffuse: Color,
    pub(crate) specular: Color,
    pub(crate) shininess: f64,
    pub(crate) illumination: u8,
    pub(crate) optical_density: f64,
    pub(crate) dissolve: f64,
}

impl Material {
    pub(crate) fn lambert() -> Self {
        Material {
            ambient: [0.2, 0.2, 0.2].into(),
            diffuse: [0.8, 0.8, 0.8].into(),
            specular: [0.0, 0.0, 0.0].into(),
            shininess: 10.,
            illumination: 1,
            optical_density: 1.0,
            dissolve: 1.0,
        }
    }

    pub(crate) fn reflective() -> Self {
        Material {
            ambient: [0.2, 0.2, 0.2].into(),
            diffuse: [0.8, 0.8, 0.8].into(),
            specular: [0.5, 0.5, 0.5].into(),
            shininess: 10.,
            illumination: 2,
            optical_density: 1.0,
            dissolve: 1.0,
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
            illumination: mat.illumination_model.unwrap_or(2),
            optical_density: mat.optical_density.into(),
            dissolve: mat.dissolve.into(),
        }
    }
}
