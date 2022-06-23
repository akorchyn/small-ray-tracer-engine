use std::{
    io::{BufRead, Result},
    path::PathBuf,
    rc::Rc,
};

use crate::{
    basic_geometry::{normal::Normal, point::Point, triangle::Triangle, vector::Vector},
    ray_tracer::RayTracable,
};

use super::Input;

pub(crate) struct ObjectFile {
    path: PathBuf,
}

impl ObjectFile {
    pub(crate) fn new(path: PathBuf) -> ObjectFile {
        ObjectFile { path }
    }
}

impl Input for ObjectFile {
    fn load(&self) -> Result<Vec<Rc<dyn RayTracable>>> {
        let file = std::fs::File::open(&self.path)?;
        let reader = std::io::BufReader::new(file);
        let mut result: Vec<Rc<dyn RayTracable>> = vec![];
        let mut points = vec![];
        let mut normals = vec![];
        let mut input_vector = vec![];

        println!("Loading data from obj file...");
        for (i, l) in reader.lines().enumerate() {
            let l = l?;
            let mut iterator = l.split_whitespace();

            match iterator.next() {
                Some("v") => {
                    let x = iterator.next().unwrap().parse::<f64>().unwrap();
                    let y = iterator.next().unwrap().parse::<f64>().unwrap();
                    let z = iterator.next().unwrap().parse::<f64>().unwrap();
                    points.push(Point::new(x, y, z));
                }
                Some("vn") => {
                    let x = iterator.next().unwrap().parse::<f64>().unwrap();
                    let y = iterator.next().unwrap().parse::<f64>().unwrap();
                    let z = iterator.next().unwrap().parse::<f64>().unwrap();
                    normals.push(Vector::new(x, y, z).normalize());
                }
                Some("f") => {
                    if points.len() < 3 {
                        panic!("Not enough points to make a triangle");
                    }
                    input_vector.clear();
                    for point in iterator {
                        if !point.contains('/') {
                            let point_index = point.parse::<usize>().unwrap();
                            input_vector.push((points[point_index - 1], None));
                        } else {
                            let (p1, n1) = process_point(point);
                            let data = if let Some(n2) = n1 {
                                (points[p1 - 1], Some(normals[n2 - 1]))
                            } else {
                                (points[p1 - 1], None)
                            };
                            input_vector.push(data);
                        }
                    }

                    match input_vector.len() {
                        3 => {
                            let triangle = get_triangle(&input_vector[..3]);
                            result.push(Rc::new(triangle));
                        }
                        4 => {
                            let triangle = get_triangle(&input_vector[..3]);
                            result.push(Rc::new(triangle));
                            input_vector.remove(1);
                            let triangle = get_triangle(&input_vector[..3]);
                            result.push(Rc::new(triangle));
                        }
                        _ => {
                            panic!("Currently only triangles and squares are supported, but received {} points at line {}", input_vector.len(), i + 1);
                        }
                    }
                }
                _ => {}
            }
        }
        println!("Loaded {} objects", result.len());

        return Ok(result);

        fn process_point(line: &str) -> (usize, Option<usize>) {
            let mut iter = line.split('/');
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.nth(1).map(|i| i.parse::<usize>().unwrap()),
            )
        }

        fn get_triangle(data: &[(Point, Option<Normal>)]) -> Triangle {
            if data[0].1.is_some() {
                Triangle::with_normals(
                    data[0].0,
                    data[0].1.unwrap(),
                    data[1].0,
                    data[1].1.unwrap(),
                    data[2].0,
                    data[2].1.unwrap(),
                )
            } else {
                Triangle::new(data[0].0, data[1].0, data[2].0)
            }
        }
    }
}
