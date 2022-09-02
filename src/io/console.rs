use crate::{io::Output, ray_tracer::color::Color};

pub(crate) struct Console {}

impl Output for Console {
    fn dump(&mut self, _: &[Color], _: usize, _: usize) -> anyhow::Result<()> {
        todo!("I have to figure out color implementations for console")
        // let mut stream = BufWriter::with_capacity(width * height, std::io::stdout());
        // for y in 0..height {
        //     for x in 0..width {
        //         let index = y * width + x;
        //         let intensity = buff[index];
        //         let char = match intensity {
        //             l if l > 0.0 && l < 0.2 => b'.',
        //             l if l > 0.2 && l < 0.5 => b'*',
        //             l if l > 0.5 && l < 0.8 => b'O',
        //             l if l > 0.8 => b'#',
        //             _ => b' ',
        //         };
        //         stream.write_all(&[char])?;
        //     }
        //     stream.write_all(b"\n")?;
        // }
        // stream.flush()?;
        // Ok(())
    }

    fn process(&mut self, mut ray_tracer: crate::ray_tracer::RayTracer) -> anyhow::Result<()> {
        ray_tracer.render(self)
    }
}
