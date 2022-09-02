use crate::ray_tracer::{color::Color, material::Material, object::Object, RayTracer};
use std::path::PathBuf;

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
    fn process(&mut self, ray_tracer: RayTracer) -> anyhow::Result<()>;
    fn dump(&mut self, buff: &[Color], width: usize, height: usize) -> anyhow::Result<()>;
}

pub(crate) trait Input {
    fn load(&self) -> anyhow::Result<(Vec<Object>, Vec<Material>)>;
}
