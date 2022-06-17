pub(crate) mod camera;
pub(crate) mod light;
pub(crate) mod scene;
pub(crate) mod viewframe;

use camera::Camera;
use scene::Scene;

pub(crate) struct RayTracer {
    scene: Scene,
    camera: Camera,
    width: usize,
    height: usize,
}

impl RayTracer {
    pub(crate) fn new(scene: Scene, camera: Camera, width: usize, height: usize) -> RayTracer {
        RayTracer {
            scene,
            camera,
            width,
            height,
        }
    }

    pub(crate) fn render_into_console(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let ray = self
                    .camera
                    .ray_for_pixel(x, self.height - y, self.width, self.height);
                let distance = self
                    .scene
                    .objects()
                    .iter()
                    .flat_map(|object| object.intersect(&ray))
                    .fold(f64::INFINITY, |a, b| a.min(b));

                if distance < f64::INFINITY {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
