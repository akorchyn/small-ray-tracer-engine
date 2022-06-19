use std::io::Result;

pub(crate) mod console;
pub(crate) mod ppm_image;

pub(crate) trait Output {
    fn dump(&self, buff: &[f64], width: usize, height: usize) -> Result<()>;
}
