mod basic_geometry;
mod io;
mod ray_tracer;

use std::path::PathBuf;

use basic_geometry::alighned_box::AlighnedBox;
use basic_geometry::normal::Normal;
use basic_geometry::point::Point;
use ray_tracer::camera::Camera;
use ray_tracer::light::DirectedLight;
use ray_tracer::scene::Scene;
use ray_tracer::viewframe::ViewFrame;
use ray_tracer::RayTracer;

fn main() {
    let object = AlighnedBox::from_dimensions(Point::new(0., 5., 0.), 10., 10., 10.);
    let mut scene = Scene::new();
    scene.add_object(Box::new(object));
    scene.add_light(DirectedLight::new(Normal::new(0.0, 0.0, 1.0)));
    scene.add_light(DirectedLight::new(Normal::new(0.0, 0.7, 0.0)));
    scene.add_light(DirectedLight::new(Normal::new(-0.4, 0.0, 0.0)));
    let viewframe = ViewFrame::new(Point::new(-40.0, 35.0, 50.0), 40.0, 40.0);
    let camera = Camera::new(Point::new(-40.0, 35.0, 80.0), viewframe);
    let ray_tracer = RayTracer::new(scene, camera, 500, 500);
    ray_tracer
        .render(io::ppm_image::PPMImage::new(PathBuf::from("./image.ppm")))
        .unwrap();
}
