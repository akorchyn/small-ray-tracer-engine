use std::fs::File;
use std::io::BufWriter;
use std::io::Result;
use std::io::Write;
use std::path::PathBuf;

use crate::io::Output;

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
    fn dump(&self, buff: &[f64], width: usize, height: usize) -> Result<()> {
        let stream = File::create(&self.file_path)?;
        let mut stream = BufWriter::new(stream);
        self.write_header(width, height, &mut stream)?;
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let intensity = buff[index];
                let r = (intensity * 255.0) as u8;
                stream.write(&[r, r, r])?;
            }
            // stream.write(b"\n")?;
        }

        Ok(())
    }
}
