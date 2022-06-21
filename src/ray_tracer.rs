pub(crate) mod camera;
pub(crate) mod light;
pub(crate) mod scene;
pub(crate) mod viewframe;

use camera::Camera;
use scene::Scene;

use crate::basic_geometry::normal::Normal;
use crate::basic_geometry::point::Point;
use crate::basic_geometry::ray::Ray;
use crate::basic_geometry::Intersect;
use crate::basic_geometry::Intersection;
use crate::basic_geometry::NormalAtPoint;
use crate::basic_geometry::Transform;

use crate::io::Output;

pub(crate) trait RayTracable: Intersect + NormalAtPoint + Transform {}
impl<T> RayTracable for T where T: Intersect + NormalAtPoint + Transform {}

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
                let traced = self.trace(&ray);

                if let Some((index, intersection)) = traced {
                    let object = self.scene.objects().get(index).unwrap();
                    let point = ray.at(intersection.distance().unwrap());
                    let normal = object.normal_at_point(&point, intersection);
                    let intensity = self.light_value(normal, point, index);
                    buff[y * self.width + x] = intensity;
                }
            }
        }
        output.dump(&buff, self.width, self.height)
    }

    fn is_any_object_blocking(&self, ray: &Ray, object_id: usize) -> bool {
        self.scene.objects().iter().enumerate().any(|(id, object)| {
            if id == object_id {
                return false;
            }
            if let Some(distance) = object.intersect(ray).distance() {
                distance > 0.
            } else {
                false
            }
        })
    }

    fn light_value(&self, normal: Normal, intersection_point: Point, object_id: usize) -> f64 {
        self.scene
            .lights()
            .iter()
            .map(|light| {
                let light_dir = (light.position - intersection_point).normalize();
                let ray = Ray::new(intersection_point, light_dir);
                if self.is_any_object_blocking(&ray, object_id) {
                    (light_dir.dot(normal) * 0.5).max(0.0)
                } else {
                    light_dir.dot(normal).max(0.0)
                }
            })
            .sum::<f64>()
            .min(1.0)
    }

    fn trace(&self, ray: &Ray) -> Option<(usize, Intersection)> {
        self.scene
            .objects()
            .iter()
            .enumerate()
            .map(|(i, object)| (i, object.intersect(ray)))
            .filter(|&(_, intesection)| intesection != Intersection::DoesNotIntersect)
            .min_by(|&(_, a), &(_, b)| {
                a.distance()
                    .unwrap()
                    .partial_cmp(&b.distance().unwrap())
                    .expect("Expected non NAN distance")
            })
    }
}
