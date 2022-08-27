use std::cell::{Ref, RefCell};
use std::io::Result;
use std::path::PathBuf;
use std::rc::Rc;

use super::object::Object;
use super::ObjectContainer;
use crate::basic_geometry::ray::Ray;
use crate::basic_geometry::{Intersect, Intersection};
use crate::complex_structures::bvh::BVHTree;
use crate::io::Input;

use super::light::Light;

pub(crate) struct LinearTracer {
    objects: Vec<Object>,
}

impl LinearTracer {
    pub(crate) fn new(objects: Vec<Object>) -> LinearTracer {
        LinearTracer { objects }
    }

    pub(crate) fn from_obj_file(path: PathBuf) -> Result<LinearTracer> {
        let loader = crate::io::obj_file::ObjectFile::new(path);
        Ok(LinearTracer::new(loader.load()?))
    }
}

impl ObjectContainer for LinearTracer {
    fn trace(&self, ray: &Ray) -> Option<(usize, Intersection)> {
        self.objects
            .iter()
            .enumerate()
            .flat_map(|(i, object)| object.intersect(ray).map(|intersection| (i, intersection)))
            .min_by(|&(_, a), &(_, b)| a.distance().total_cmp(&b.distance()))
    }

    fn object_by_index(&self, index: usize) -> &Object {
        &self.objects[index]
    }
}

pub(crate) enum Tracing {
    Linear,
    BVH,
}

pub(crate) struct Scene {
    objects: Box<dyn ObjectContainer>,
    lights: Vec<Light>,
}

impl Scene {
    pub(crate) fn new(objects: Box<dyn ObjectContainer>) -> Scene {
        Scene {
            objects,
            lights: Vec::new(),
        }
    }

    pub(crate) fn from_obj_file(path: PathBuf, t: Tracing) -> Result<Scene> {
        let tracer: Box<dyn ObjectContainer> = match t {
            Tracing::Linear => Box::new(LinearTracer::from_obj_file(path)?),
            Tracing::BVH => Box::new(BVHTree::from_obj_file(path)?),
        };

        Ok(Scene {
            objects: tracer,
            lights: Vec::new(),
        })
    }

    pub(crate) fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub(crate) fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub(crate) fn objects(&self) -> &dyn ObjectContainer {
        self.objects.as_ref()
    }
}
