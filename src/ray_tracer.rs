pub(crate) mod camera;
pub(crate) mod light;
pub(crate) mod scene;
pub(crate) mod viewframe;

use std::cell::Ref;

use camera::Camera;
use scene::Scene;

use crate::basic_geometry::normal::Normal;
use crate::basic_geometry::point::Point;
use crate::basic_geometry::ray::Ray;
use crate::basic_geometry::Intersect;
use crate::basic_geometry::Intersection;
use crate::basic_geometry::NormalAtPoint;
use crate::basic_geometry::Transform;

use crate::complex_structures::BoundingBox;
use crate::io::Output;

pub(crate) trait RayTracable: Intersect + NormalAtPoint + Transform + BoundingBox {}

impl<T> RayTracable for T where T: Intersect + NormalAtPoint + Transform + BoundingBox {}

pub(crate) trait ObjectContainer {
    fn trace(&self, ray: &Ray) -> Option<(usize, Intersection)>;
    fn object_by_index(&self, index: usize) -> Ref<dyn RayTracable>;
}

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

    pub(crate) fn render(&self, output: impl Output) -> Result<(), std::io::Error> {
        let mut buff = vec![-1.0; self.width * self.height];
        for y in 0..self.height {
            println!("Ray-tracing row: {}/{}", y, self.height);
            for x in 0..self.width {
                let ray = self
                    .camera
                    .ray_for_pixel(x, self.height - y, self.width, self.height);
                let traced = self.scene.objects().trace(&ray);

                if let Some((index, intersection)) = traced {
                    let object = self.scene.objects().object_by_index(index);
                    let point = ray.at(intersection.distance());
                    let normal = object.normal_at_point(&point, intersection);
                    let intensity = self.light_value(normal, point);
                    buff[y * self.width + x] = intensity;
                }
            }
        }
        output.dump(&buff, self.width, self.height)
    }
    fn light_value(&self, normal: Normal, intersection_point: Point) -> f64 {
        self.scene
            .lights()
            .iter()
            .map(|light| {
                let light_dir = (light.position - intersection_point).normalize();
                let ray = Ray::new(intersection_point, light_dir);
                let ray = Ray::new(ray.at(1.0), light_dir);
                if self.scene.objects().trace(&ray).is_some() {
                    (light_dir.dot(normal) * 0.5).max(0.0)
                } else {
                    light_dir.dot(normal).max(0.0)
                }
            })
            .sum::<f64>()
            .min(1.0)
    }
}
