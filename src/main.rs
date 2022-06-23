mod basic_geometry;
mod complex_structures;
mod io;
mod ray_tracer;

use std::ffi::OsStr;
use std::path::PathBuf;

use basic_geometry::point::Point;
use basic_geometry::{Axis, Transformation};
use io::Input;
use ray_tracer::camera::Camera;
use ray_tracer::light::Light;
use ray_tracer::scene::{Scene, Tracing};
use ray_tracer::viewframe::ViewFrame;
use ray_tracer::{ObjectContainer, RayTracer};

fn parse_args() -> (PathBuf, PathBuf, Tracing) {
    const HELP_MSG: &str = "./graphics --source=path_to_object.obj --output=path_to_result.ppm\n 
The ratracer takes two arguments: the input file and the output file.
The input file is a object file in the Wavefront OBJ format.
The output file is a image fiile in the PPM file format.";

    let mut source: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut tracing = Tracing::BVH;
    for arg in std::env::args() {
        if arg == "--help" {
            println!("{}", HELP_MSG);
            std::process::exit(0);
        } else if arg.starts_with("--source=") {
            if let Some(path) = arg.split('=').nth(1) {
                let path = PathBuf::from(path);
                if Some(OsStr::new("obj")) == path.extension() {
                    if path.exists() {
                        source = Some(path);
                    } else {
                        println!("The source file does not exist.\n\n{}", HELP_MSG);
                        std::process::exit(1);
                    }
                } else {
                    println!("Incorrect input file format\n\n{}", HELP_MSG);
                    std::process::exit(0);
                }
            }
        } else if arg.starts_with("--output=") {
            if let Some(path) = arg.split('=').nth(1) {
                let path = PathBuf::from(path);
                if Some(OsStr::new("ppm")) == path.extension() {
                    output = Some(path);
                } else {
                    println!("Incorrect output file format\n\n{}", HELP_MSG);
                    std::process::exit(0);
                }
            }
        } else if arg.starts_with("--without-tree") {
            tracing = Tracing::Linear;
        }
    }

    if source.is_none() || output.is_none() {
        println!("All required arguments is not provided.\n\n{}", HELP_MSG);
        std::process::exit(0);
    }
    (source.unwrap(), output.unwrap(), tracing)
}

fn main() {
    let (source, output, tracing) = parse_args();
    let loader = io::obj_file::ObjectFile::new(source);
    match loader.load() {
        Err(e) => {
            println!("Failed to process object file:\n{}", e);
            std::process::exit(1);
        }
        Ok(objects) => {
            for object in objects.iter() {
                let mut object = object.borrow_mut();
                object.transform(Transformation::Rotation(Axis::Y, 90.0));
                // object.transform(Transformation::Rotation(Axis::Z, 90.0));
            }
            let tracer: Box<dyn ObjectContainer> = match tracing {
                Tracing::BVH => Box::new(complex_structures::bvh::BVHTree::new(objects, 32)),
                Tracing::Linear => Box::new(ray_tracer::scene::LinearTracer::new(objects)),
            };
            let mut scene = Scene::new(tracer);
            scene.add_light(Light::new(Point::new(-50.0, 100.0, 150.0)));

            let viewframe = ViewFrame::new(Point::new(75.0, 00.0, 200.0), 48.0, 27.0);
            let camera = Camera::new(Point::new(75.0, 00.0, 250.0), viewframe);
            let ray_tracer = RayTracer::new(scene, camera, 3840, 2160);
            ray_tracer
                .render(io::ppm_image::PPMImage::new(output))
                .unwrap();
        }
    }
}
