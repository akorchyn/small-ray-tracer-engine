use crate::basic_geometry::Intersect;
use crate::ray_tracer::light::Light;

pub(crate) struct Scene {
    objects: Vec<Box<dyn Intersect>>,
    lights: Vec<Light>,
}

impl Scene {
    pub(crate) fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub(crate) fn add_object(&mut self, object: Box<dyn Intersect>) {
        self.objects.push(object);
    }

    pub(crate) fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub(crate) fn objects(&self) -> &Vec<Box<dyn Intersect>> {
        &self.objects
    }

    pub(crate) fn lights(&self) -> &Vec<Light> {
        &self.lights
    }
}
