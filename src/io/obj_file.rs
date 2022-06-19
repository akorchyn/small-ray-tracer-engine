use std::{
    io::{BufRead, Result},
    path::PathBuf,
};

use crate::{
    basic_geometry::{normal::Normal, point::Point, triangle::Triangle, vector::Vector},
    ray_tracer::scene::Scene,
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
    fn load(&self) -> Result<Scene> {
        let file = std::fs::File::open(&self.path)?;
        let reader = std::io::BufReader::new(file);
        let mut scene = Scene::new();
        let mut points = vec![];
        let mut normals = vec![];

        for (i, l) in reader.lines().enumerate() {
            let l = l?;
            let mut iterator = l.split_whitespace();
            println!("Loading data {}: {}", i, &l);

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
                    let mut array = [(Point::new(0.0, 0.0, 0.0), Option::<Normal>::None); 3];

                    for i in 0..3 {
                        let point_line = iterator.next().unwrap();
                        if point_line.find("/").is_none() {
                            let point_index = point_line.parse::<usize>().unwrap();
                            array[i].0 = points[point_index - 1];
                        } else {
                            let (p1, n1) = process_point(iterator.next().unwrap());
                            array[i].0 = points[p1 - 1];
                            if let Some(n2) = n1 {
                                array[i].1 = Some(normals[n2 - 1]);
                            }
                        }
                    }

                    let triangle = if array[0].1.is_some() {
                        Triangle::with_normals(
                            array[0].0,
                            array[0].1.unwrap(),
                            array[1].0,
                            array[1].1.unwrap(),
                            array[2].0,
                            array[2].1.unwrap(),
                        )
                    } else {
                        Triangle::new(array[0].0, array[1].0, array[2].0)
                    };
                    scene.add_object(Box::new(triangle));
                }
                _ => {}
            }
        }

        return Ok(scene);

        fn process_point(line: &str) -> (usize, Option<usize>) {
            let mut iter = line.split('/');
            (
                iter.nth(0).unwrap().parse::<usize>().unwrap(),
                iter.nth(1).map(|i| i.parse::<usize>().unwrap()),
            )
        }
    }
}
