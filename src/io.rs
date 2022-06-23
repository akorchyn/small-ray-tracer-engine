use std::cell::RefCell;
use std::io::Result;
use std::rc::Rc;

use crate::ray_tracer::RayTracable;

pub(crate) mod console;
pub(crate) mod obj_file;
pub(crate) mod ppm_image;

pub(crate) trait Output {
    fn dump(&self, buff: &[f64], width: usize, height: usize) -> Result<()>;
}

pub(crate) trait Input {
    fn load(&self) -> Result<Vec<Rc<RefCell<dyn RayTracable>>>>;
}
