mod basic_geometry;
mod complex_structures;
mod io;
mod ray_tracer;

use std::cell::RefCell;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::rc::Rc;

use basic_geometry::alighned_box::AlighnedBox;
use basic_geometry::normal::Normal;
use basic_geometry::plane::Plane;
use basic_geometry::point::Point;
use basic_geometry::sphere::Sphere;
use io::Input;
use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::light::Light;
use ray_tracer::object::Object;
use ray_tracer::scene::{Scene, Tracing};
use ray_tracer::viewframe::ViewFrame;
use ray_tracer::{ObjectContainer, RayTracer};

use crate::io::OutputType;

type IsSphereNeeded = bool;

fn parse_args() -> (PathBuf, OutputType, Tracing, IsSphereNeeded) {
    const HELP_MSG: &str = "./graphics --source=path_to_object.obj [--output=path_to_result.ppm, --windowed, --console]\n 
The ratracer takes two arguments: the input file and the output file.
The input file is a object file in the Wavefront OBJ format.
The output is either a file or one of the other output formats (window, console).
Optional arguments:
--add-sphere - add predefined sphere";

    let mut source: Option<PathBuf> = None;
    let mut output: Option<OutputType> = None;
    let mut tracing = Tracing::BVH;
    let mut add_sphere = false;
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
                    output = Some(OutputType::Image(path));
                } else {
                    println!("Incorrect output file format\n\n{}", HELP_MSG);
                    std::process::exit(0);
                }
            }
        } else if arg.eq("--without-tree") {
            tracing = Tracing::Linear;
        } else if arg.eq("--add-sphere") {
            add_sphere = true;
        } else if arg.eq("--console") {
            output = Some(OutputType::Console);
        }

        #[cfg(feature = "windowed")]
        if arg.starts_with("--windowed") {
            output = Some(OutputType::Window((500, 500)));
        }
    }

    if source.is_none() || output.is_none() {
        println!("All required arguments is not provided.\n\n{}", HELP_MSG);
        std::process::exit(0);
    }
    (source.unwrap(), output.unwrap(), tracing, add_sphere)
}

fn main() {
    let (source, output, tracing, sphere) = parse_args();
    let loader = io::obj_file::ObjectFile::new(source);
    match loader.load() {
        Err(e) => {
            println!("Failed to process object file:\n{}", e);
            std::process::exit(1);
        }
        Ok(mut objects) => {
            if sphere {
                objects.push(Object::reflection(
                    Rc::new(RefCell::new(Sphere::new(Point::new(20., 20., 20.0), 5.0))),
                    0.3,
                ));
                // objects.push(Object::lambert(Rc::new(RefCell::new(AlighnedBox::new(
                //     Point::new(-100., -100.0, -100.0),
                //     Point::new(100.0, -100.0, 100.0),
                // )))));
            }
            let tracer: Box<dyn ObjectContainer> = match tracing {
                Tracing::BVH => Box::new(complex_structures::bvh::BVHTree::new(objects, 1)),
                Tracing::Linear => Box::new(ray_tracer::scene::LinearTracer::new(objects)),
            };
            let mut scene = Scene::new(tracer);
            scene.add_light(Light::Point(
                Point::new(0.0, 100.0, 100.0),
                Color::white(),
                0.7,
            ));
            scene.add_light(Light::Environment(Color::red(), 0.15));
            scene.add_light(Light::Directed(
                Normal::new(-1., 0., 0.),
                Color::blue(),
                0.15,
            ));

            let viewframe = ViewFrame::new(Point::new(0.0, 0.0, 250.0), 25.0, 25.0);
            let camera = Camera::new(Point::new(0.0, 0.0, 275.0), viewframe);
            let ray_tracer = RayTracer::new(scene, camera, 500, 500);
            let mut output = output.create_handler();
            output.process(ray_tracer).unwrap()
        }
    }
}
