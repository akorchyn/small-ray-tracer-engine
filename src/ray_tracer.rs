pub(crate) mod camera;
pub(crate) mod color;
pub(crate) mod light;
pub(crate) mod material;
pub(crate) mod object;
pub(crate) mod scene;
pub(crate) mod texture_loader;
pub(crate) mod viewframe;

use camera::Camera;
use material::Material;
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
use self::texture_loader::TextureLoader;

const MIRROR_RECURSION_LIMIT: u32 = 4;

const DEFAULT_BACKGROUND_COLOR: Color = Color::new(0.18, 0.39, 0.);

pub(crate) trait RayTracable: Intersect + NormalAtPoint + Transform + BoundingBox {}

impl<T> RayTracable for T where T: Intersect + NormalAtPoint + Transform + BoundingBox {}

pub(crate) trait ObjectContainer {
    fn trace(&self, ray: &Ray) -> Option<(usize, Intersection)>;
    fn object_by_index(&self, index: usize) -> &Object;
}

pub(crate) struct RayTracer {
    scene: Scene,
    texture_loader: TextureLoader,
    camera: Camera,
    width: usize,
    height: usize,
}

impl RayTracer {
    pub(crate) fn new(scene: Scene, camera: Camera, width: usize, height: usize) -> RayTracer {
        RayTracer {
            scene,
            texture_loader: TextureLoader::new(),
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

    pub(crate) fn render(&mut self, output: &mut dyn Output) -> anyhow::Result<()> {
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
            let intersection_point = ray.at(intersection.distance());
            let normal = object.normal_at_point(&intersection_point, intersection);
            let material = self.scene.materials(object.material_id).clone();
            let color = self.get_color(intersection_point, normal, &material, ray);
            color
        } else {
            DEFAULT_BACKGROUND_COLOR
        }
    }

    fn get_color(
        &self,
        intersection_point: Point,
        normal: Normal,
        material: &Material,
        ray: Ray,
    ) -> Color {
        self.scene
            .lights()
            .iter()
            .map(|light| match *light {
                Light::Environment(color, coof) => color * coof * material.ambient,
                Light::Point(point, color, coof)
                    if !self.is_shadowed(
                        intersection_point,
                        (point - intersection_point).normalize(),
                    ) =>
                {
                    let light_dir = (intersection_point - point).normalize(); // In direction from Light to Intersection
                    RayTracer::phong_color(color * coof, light_dir, normal, &ray, &material)
                }
                Light::Directed(light_dir, color, coof)
                    if !self.is_shadowed(intersection_point, -light_dir) =>
                {
                    RayTracer::phong_color(color * coof, light_dir, normal, &ray, &material)
                }
                _ => Color::black(),
            })
            .sum::<Color>()
    }

    fn is_shadowed(&self, intersection_point: Point, dir_to_light: Normal) -> bool {
        let ray = Ray::new(intersection_point, dir_to_light);
        let ray = Ray::new(ray.at(1e-4), dir_to_light);
        self.scene.objects().trace(&ray).is_some()
    }

    fn phong_color(
        intensity: Color,
        light_dir: Normal,
        normal: Normal,
        ray: &Ray,
        material: &Material,
    ) -> Color {
        let diffuse = intensity * normal.dot(-light_dir).max(0.0) * material.diffuse;

        let reflection_light = Normal::reflect(normal, light_dir);
        let specular = intensity
            * reflection_light
                .dot(-ray.direction)
                .abs()
                .powf(material.shininess.into())
            * material.specular;

        diffuse + specular
    }
}
