use super::light::DirectedLight;
use super::RayTracable;

pub(crate) struct Scene {
    objects: Vec<Box<dyn RayTracable>>,
    lights: Vec<DirectedLight>,
}

impl Scene {
    pub(crate) fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub(crate) fn add_object(&mut self, object: Box<dyn RayTracable>) {
        self.objects.push(object);
    }

    pub(crate) fn add_light(&mut self, light: DirectedLight) {
        self.lights.push(light);
    }

    pub(crate) fn objects(&self) -> &Vec<Box<dyn RayTracable>> {
        &self.objects
    }

    pub(crate) fn lights(&self) -> &Vec<DirectedLight> {
        &self.lights
    }
}
