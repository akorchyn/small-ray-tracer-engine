use std::fs::File;
use std::io::BufWriter;
use std::io::Result;
use std::io::Write;
use std::path::PathBuf;

use crate::io::Output;
use crate::ray_tracer::color::Color;

pub(crate) struct PPMImage {
    file_path: PathBuf,
}

impl PPMImage {
    pub(crate) fn new(file_path: PathBuf) -> PPMImage {
        PPMImage { file_path }
    }

    fn write_header(
        &self,
        width: usize,
        height: usize,
        writer: &mut BufWriter<File>,
    ) -> Result<()> {
        writer.write_all(b"P6\n")?;
        writer.write_all(format!("{} {}\n", width, height).as_bytes())?;
        writer.write_all(b"255\n")?;
        Ok(())
    }
}

impl Output for PPMImage {
    fn dump(&mut self, buff: &[Color], width: usize, height: usize) -> anyhow::Result<()> {
        let stream = File::create(&self.file_path)?;
        let mut stream = BufWriter::new(stream);
        self.write_header(width, height, &mut stream)?;
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let color = buff[index];
                stream.write_all(&color.rgb())?;
            }
        }

        Ok(())
    }

    fn process(&mut self, mut ray_tracer: crate::ray_tracer::RayTracer) -> anyhow::Result<()> {
        Ok(ray_tracer.render(self)?)
    }
}
