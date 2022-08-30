use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader, Result},
    path::PathBuf,
    rc::Rc,
};

use tobj::{Material, Model};

use crate::{
    basic_geometry::{normal::Normal, point::Point, triangle::Triangle, vector::Vector},
    ray_tracer::{object::Object, RayTracable},
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
    fn load(&self) -> anyhow::Result<(Vec<Object>, Vec<Material>)> {
        let (models, materials) = tobj::load_obj(
            &self.path,
            &tobj::LoadOptions {
                triangulate: true,
                ignore_lines: true,
                ignore_points: true,
                single_index: true,
                ..Default::default()
            },
        )?;

        let mut materials = materials?;
        let lambert_id = materials.len();
        materials.push(Material {
            ambient: [0.2, 0.2, 0.2],
            diffuse: [0.8, 0.8, 0.8],
            specular: [1.0, 1.0, 1.0],
            shininess: 100.,
            ..Default::default()
        });

        let data = models
            .into_iter()
            .flat_map(|model| {
                let size = model.mesh.indices.len() / 3;
                let mut result = vec![];
                result.reserve(size);
                for i in 0..size {
                    let i = 3 * i;
                    let (i1, i2, i3) = (
                        model.mesh.indices[i] as usize,
                        model.mesh.indices[i + 1] as usize,
                        model.mesh.indices[i + 2] as usize,
                    );

                    let texture_id = model.mesh.material_id.unwrap_or(lambert_id);

                    let (point1, point2, point3) = (
                        get_point(&model.mesh.positions[i1 * 3..]),
                        get_point(&model.mesh.positions[i2 * 3..]),
                        get_point(&model.mesh.positions[i3 * 3..]),
                    );
                    let triangle = if let Some(_) = model.mesh.normals.get(i1 * 3) {
                        Triangle::with_normals(
                            point1,
                            get_normal(&model.mesh.normals[i1 * 3..]),
                            point2,
                            get_normal(&model.mesh.normals[i2 * 3..]),
                            point3,
                            get_normal(&model.mesh.normals[i3 * 3..]),
                        )
                    } else {
                        Triangle::new(point1, point2, point3)
                    };
                    result.push(Object::new(Rc::new(RefCell::new(triangle)), texture_id));
                }
                result
            })
            .collect::<Vec<_>>();
        Ok((data, materials))
    }
}

fn get_point(slice: &[f32]) -> Point {
    Point::new(slice[0] as f64, slice[1] as f64, slice[2] as f64)
}

fn get_normal(slice: &[f32]) -> Normal {
    Normal::new(slice[0] as f64, slice[1] as f64, slice[2] as f64)
}
