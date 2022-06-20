use std::path::PathBuf;

use crate::io::Input;

use super::light::Light;
use super::RayTracable;

pub(crate) struct Scene {
    objects: Vec<Box<dyn RayTracable>>,
    lights: Vec<Light>,
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

    pub(crate) fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub(crate) fn objects(&self) -> &Vec<Box<dyn RayTracable>> {
        &self.objects
    }

    pub(crate) fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub(crate) fn from_obj_file(path: PathBuf) -> Result<Scene, std::io::Error> {
        let loader = crate::io::obj_file::ObjectFile::new(path);
        loader.load()
    }
}
