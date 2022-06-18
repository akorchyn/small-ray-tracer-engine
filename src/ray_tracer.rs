pub(crate) mod camera;
pub(crate) mod light;
pub(crate) mod scene;
pub(crate) mod viewframe;

use camera::Camera;
use scene::Scene;

use crate::basic_geometry::normal::Normal;
use crate::basic_geometry::ray::Ray;
use crate::basic_geometry::Intersect;
use crate::basic_geometry::NormalAtPoint;

pub(crate) trait RayTracable: Intersect + NormalAtPoint {}
impl<T> RayTracable for T where T: Intersect + NormalAtPoint {}

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
                let traced = self.trace(&ray);

                if let Some((index, distance)) = traced {
                    let object = self.scene.objects().get(index).unwrap();
                    let point = ray.at(distance);
                    let normal = object.normal_at_point(&point);
                    let intensity = self.light_value_at_normal(&normal);
                    // let intensity = 1.0;
                    let char = match intensity {
                        l if l > 0.0 && l < 0.2 => '.',
                        l if l > 0.2 && l < 0.5 => '*',
                        l if l > 0.5 && l < 0.8 => 'O',
                        l if l > 0.8 => '#',
                        _ => ' ',
                    };
                    print!("{}", char);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn light_value_at_normal(&self, normal: &Normal) -> f64 {
        self.scene
            .lights()
            .iter()
            .map(|light| light.intensity_at_normal(normal))
            .sum()
    }

    fn trace(&self, ray: &Ray) -> Option<(usize, f64)> {
        self.scene
            .objects()
            .iter()
            .enumerate()
            .flat_map(|(i, object)| {
                object
                    .intersect(&ray)
                    .and_then(|distance| Some((i, distance)))
            })
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Expected non NAN distance"))
    }
}
