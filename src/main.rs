mod basic_geometry;
mod ray_tracer;

use basic_geometry::point::Point;
use basic_geometry::sphere::Sphere;
use ray_tracer::camera::Camera;
use ray_tracer::scene::Scene;
use ray_tracer::viewframe::ViewFrame;
use ray_tracer::RayTracer;

fn main() {
    let mut scene = Scene::new();
    scene.add_object(Box::new(Sphere::new(Point::new(0.0, 10.0, 50.0), 5.)));
    scene.add_object(Box::new(Sphere::new(Point::new(-10.0, 0.0, 50.0), 5.)));
    scene.add_object(Box::new(Sphere::new(Point::new(10.0, 0.0, 50.0), 5.)));
    scene.add_object(Box::new(Sphere::new(Point::new(-5.0, -10.0, 50.0), 5.)));
    scene.add_object(Box::new(Sphere::new(Point::new(5.0, -10.0, 50.0), 5.)));

    let viewframe = ViewFrame::new(Point::new(0.0, 0.0, 50.0), 50.0, 50.0);

    let camera = Camera::new(Point::new(0.0, 0.0, 0.0), viewframe);

    let ray_tracer = RayTracer::new(scene, camera, 50, 50);
    ray_tracer.render_into_console();
}
