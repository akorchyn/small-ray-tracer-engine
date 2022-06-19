mod basic_geometry;
mod io;
mod ray_tracer;

use std::path::PathBuf;

use basic_geometry::normal::Normal;
use basic_geometry::point::Point;
use basic_geometry::triangle;
use ray_tracer::camera::Camera;
use ray_tracer::light::DirectedLight;
use ray_tracer::scene::Scene;
use ray_tracer::viewframe::ViewFrame;
use ray_tracer::RayTracer;

fn main() {
    let object = triangle::Triangle::new(
        Point::new(-10., 0., 0.),
        Point::new(10., 0., 0.),
        Point::new(0., 20., 0.),
    );
    let mut scene = Scene::new();
    scene.add_object(Box::new(object));
    scene.add_light(DirectedLight::new(Normal::new(0.0, 0.0, 1.0)));
    // scene.add_light(DirectedLight::new(Normal::new(0.0, 0.7, 0.0)));
    // scene.add_light(DirectedLight::new(Normal::new(-0.4, 0.0, 0.0)));
    let viewframe = ViewFrame::new(Point::new(0.0, 5.0, 50.0), 40.0, 40.0);
    let camera = Camera::new(Point::new(0.0, 5.0, 80.0), viewframe);
    let ray_tracer = RayTracer::new(scene, camera, 500, 500);
    ray_tracer
        .render(io::ppm_image::PPMImage::new(PathBuf::from("./image.ppm")))
        .unwrap();
}
