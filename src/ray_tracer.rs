pub(crate) mod camera;
pub(crate) mod color;
pub(crate) mod light;
pub(crate) mod object;
pub(crate) mod scene;
pub(crate) mod texture;
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
use crate::basic_geometry::Transformation;

use crate::basic_geometry::vector::Vector;
use crate::complex_structures::BoundingBox;
use crate::io::Output;
use object::Object;

use self::color::Color;
use self::light::Light;
use self::texture::Texture;

const MIRROR_RECURSION_LIMIT: u32 = 4;

const DEFAULT_BACKGROUND_COLOR: Color = Color::new(45, 100, 0);

pub(crate) trait RayTracable: Intersect + NormalAtPoint + Transform + BoundingBox {}

impl<T> RayTracable for T where T: Intersect + NormalAtPoint + Transform + BoundingBox {}

pub(crate) trait ObjectContainer {
    fn trace(&self, ray: &Ray) -> Option<(usize, Intersection)>;
    fn object_by_index(&self, index: usize) -> &Object;
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

    pub(crate) fn transform_camera(&mut self, transformation: Transformation) {
        self.camera.transform(transformation)
    }

    pub(crate) fn rotation_vector(&self) -> Vector {
        self.camera.rotation_vector()
    }

    pub(crate) fn render(&self, output: &mut dyn Output) -> Result<(), std::io::Error> {
        let mut buff = vec![DEFAULT_BACKGROUND_COLOR; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let ray = self
                    .camera
                    .ray_for_pixel(x, self.height - y, self.width, self.height);
                buff[y * self.width + x] = self.get_color_for_ray(ray, 0);
            }
        }
        output.dump(&buff, self.width, self.height)
    }

    fn get_color_for_ray(&self, ray: Ray, nonce: u32) -> Color {
        let traced = self.scene.objects().trace(&ray);

        if let Some((object, intersection)) = traced {
            let object = self.scene.objects().object_by_index(object);
            let point = ray.at(intersection.distance());
            let normal = object.normal_at_point(&point, intersection);

            let color = self.light_value(normal, point);

            match object.texture {
                Texture::Reflection(coof) if nonce < MIRROR_RECURSION_LIMIT => {
                    // We need to create a new ray
                    let normal = Vector::from(normal);
                    let dir = Vector::from(ray.direction);

                    let ray = Ray::new(point, (normal * -2. * normal.dot(dir) + dir).normalize());
                    let ray = Ray::new(ray.at(1e-4), ray.direction);
                    self.get_color_for_ray(ray, nonce + 1) * coof + color * (1. - coof)
                }
                _ => color,
            }
        } else {
            DEFAULT_BACKGROUND_COLOR
        }
    }

    fn light_value(&self, normal: Normal, intersection_point: Point) -> Color {
        return self
            .scene
            .lights()
            .iter()
            .map(|light| match *light {
                Light::Environment(color, coof) => color * coof,
                Light::Point(point, color, coof) => {
                    let light_dir = (point - intersection_point).normalize();
                    get_directed_color(
                        self.scene.objects(),
                        intersection_point,
                        color * coof,
                        light_dir,
                        normal,
                    )
                }
                Light::Directed(light_dir, color, coof) => get_directed_color(
                    self.scene.objects(),
                    intersection_point,
                    color * coof,
                    -light_dir,
                    normal,
                ),
            })
            .sum::<Color>();

        fn get_directed_color(
            objects: &dyn ObjectContainer,
            intersection_point: Point,
            light_color: Color,
            dir_to_light: Normal,
            normal: Normal,
        ) -> Color {
            let ray = Ray::new(intersection_point, dir_to_light);
            let ray = Ray::new(ray.at(1e-4), dir_to_light);
            if objects.trace(&ray).is_some() {
                Color::black()
            } else {
                light_color * dir_to_light.dot(normal).max(0.0)
            }
        }
    }
}
