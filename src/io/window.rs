use crate::{
    basic_geometry::{vector::Vector, Axis, Transformation},
    ray_tracer::RayTracer,
};

use super::Output;
use std::io::Result;

use minifb::{Key, Window as WindowHandler, WindowOptions};

pub(crate) struct Window {
    window: WindowHandler,
}

impl Window {
    pub(crate) fn new(width: usize, height: usize) -> Window {
        let mut window =
            WindowHandler::new("Raytracer", width, height, WindowOptions::default()).unwrap();
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Window { window }
    }

    fn handle_events(&mut self, ray_tracer: &mut RayTracer) -> bool {
        let mut handled_event = false;
        let keys = self.window.get_keys();
        keys.iter().for_each(|key| match key {
            Key::W => {
                ray_tracer.transform_camera(Transformation::Translation(-movement(
                    ray_tracer.rotation_vector(),
                )));
                handled_event = true;
            }
            Key::S => {
                ray_tracer.transform_camera(Transformation::Translation(movement(
                    ray_tracer.rotation_vector(),
                )));
                handled_event = true;
            }
            Key::A => {
                ray_tracer
                    .transform_camera(Transformation::Translation(Vector::new(-0.5, 0.0, 0.0)));
                handled_event = true;
            }
            Key::D => {
                ray_tracer
                    .transform_camera(Transformation::Translation(Vector::new(0.5, 0.0, 0.0)));
                handled_event = true;
            }
            Key::LeftShift => {
                ray_tracer
                    .transform_camera(Transformation::Translation(Vector::new(0.0, 0.5, 0.0)));
                handled_event = true;
            }
            Key::LeftCtrl => {
                ray_tracer
                    .transform_camera(Transformation::Translation(Vector::new(0.0, -0.5, 0.0)));
                handled_event = true;
            }
            Key::Left => {
                ray_tracer.transform_camera(Transformation::Rotation(Axis::Y, 5.0));
                handled_event = true;
            }
            Key::Right => {
                ray_tracer.transform_camera(Transformation::Rotation(Axis::Y, -5.0));
                handled_event = true;
            }
            Key::Up => {
                ray_tracer.transform_camera(Transformation::Rotation(Axis::X, -5.0));
                handled_event = true;
            }
            Key::Down => {
                ray_tracer.transform_camera(Transformation::Rotation(Axis::X, 5.0));
                handled_event = true;
            }
            Key::RightCtrl => {
                ray_tracer.transform_camera(Transformation::Rotation(Axis::Z, 5.0));
                handled_event = true;
            }
            Key::RightShift => {
                ray_tracer.transform_camera(Transformation::Rotation(Axis::Z, -5.0));
                handled_event = true;
            }
            _ => {}
        });
        handled_event
    }
}

impl Output for Window {
    fn dump(&mut self, buff: &[f64], width: usize, height: usize) -> Result<()> {
        println!("{}", "Dumping...");
        let buff = buff
            .iter()
            .map(|x| {
                let color = (x * 255.0) as u32;
                if *x == -1.0 {
                    45u32 << 16 | 100u32 << 8
                } else {
                    color << 16 | color << 8 | color
                }
            })
            .collect::<Vec<_>>();
        self.window
            .update_with_buffer(&buff, width, height)
            .unwrap();
        Ok(())
    }

    fn process(&mut self, mut ray_tracer: RayTracer) -> Result<()> {
        ray_tracer.render(self)?;
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            if self.handle_events(&mut ray_tracer) {
                ray_tracer.render(self)?;
            } else {
                self.window.update();
            }
        }
        Ok(())
    }
}

fn movement(camera_angles: Vector) -> Vector {
    let movement = 0.5;
    let pitch = camera_angles.x.to_radians();
    let yaw = camera_angles.y.to_radians();

    let x = movement * yaw.sin() * pitch.cos();
    let y = movement * -pitch.sin();
    let z = movement * yaw.cos() * pitch.cos();

    Vector::new(x, y, z)
}
