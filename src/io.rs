use std::cell::RefCell;
use std::io::Result;
use std::path::PathBuf;
use std::rc::Rc;

use crate::ray_tracer::{self, RayTracable, RayTracer};

pub(crate) mod console;
pub(crate) mod obj_file;
pub(crate) mod ppm_image;
#[cfg(feature = "windowed")]
pub(crate) mod window;

pub(crate) enum OutputType {
    Console,
    Image(PathBuf),
    #[cfg(feature = "windowed")]
    Window((usize, usize)),
}

impl OutputType {
    pub(crate) fn create_handler(&self) -> Box<dyn Output> {
        match self {
            OutputType::Console => Box::new(console::Console {}),
            OutputType::Image(path) => Box::new(ppm_image::PPMImage::new(path.clone())),
            #[cfg(feature = "windowed")]
            &OutputType::Window((width, height)) => Box::new(window::Window::new(width, height)),
        }
    }
}

pub(crate) trait Output {
    fn process(&mut self, ray_tracer: RayTracer) -> Result<()>;
    fn dump(&mut self, buff: &[f64], width: usize, height: usize) -> Result<()>;
}

pub(crate) trait Input {
    fn load(&self) -> Result<Vec<Rc<RefCell<dyn RayTracable>>>>;
}
