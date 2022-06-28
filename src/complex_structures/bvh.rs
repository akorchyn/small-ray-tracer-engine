use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    path::PathBuf,
    rc::Rc,
};

use crate::{
    basic_geometry::{
        alighned_box::AlighnedBox, point::Point, ray::Ray, Axis, Intersect, Intersection,
    },
    io::Input,
    ray_tracer::{ObjectContainer, RayTracable},
};

struct BVHNode {
    index: usize,
    bounding_box: AlighnedBox,
    left: Option<usize>,
    right: Option<usize>,
    split_axis: Option<Axis>,
    start: usize,
    end: usize,
}

pub(crate) struct BVHTree {
    data: Vec<Rc<RefCell<dyn RayTracable>>>,
    nodes: Vec<BVHNode>,
    max_primitives_in_node: usize,
    root: Option<usize>,
}

struct BVHObjectInfo {
    index: usize,
    bounding_box: AlighnedBox,
    position: Point,
}

impl ObjectContainer for BVHTree {
    fn trace(&self, ray: &Ray) -> Option<(usize, Intersection)> {
        let mut queue = VecDeque::new();
        queue.push_back(self.root.unwrap());
        let mut intersections = Vec::new();
        while let Some(node_index) = queue.pop_front() {
            let node = &self.nodes[node_index];
            if node.bounding_box.intersect(&ray).is_some() {
                if node.end != node.start {
                    // self.data[node.start..node.end]
                    //     .iter()
                    //     .enumerate()
                    //     .filter_map(|(i, object)| {
                    //         object.intersect(&ray).map(|intersection| (i, intersection))
                    //     })
                    //     .min_by(|&(_, a), &(_, b)| {
                    //         a.distance()
                    //             .partial_cmp(&b.distance())
                    //             .expect("Expected non NAN distance")
                    //     })

                    for i in node.start..node.end {
                        let object = &self.data[i];
                        if let Some(intersection) = object.borrow().intersect(&ray) {
                            intersections.push((i, intersection));
                        }
                    }
                } else {
                    if let Some(left) = node.left {
                        queue.push_back(left);
                    }

                    if let Some(right) = node.right {
                        queue.push_back(right);
                    }
                }
            }
        }
        intersections.into_iter().min_by(|&(_, a), &(_, b)| {
            a.distance()
                .partial_cmp(&b.distance())
                .expect("Expected non NAN distance")
        })
    }

    fn object_by_index(&self, index: usize) -> Ref<dyn RayTracable> {
        self.data[index].borrow()
    }
}

impl BVHTree {
    pub(crate) fn from_obj_file(path: PathBuf) -> std::io::Result<BVHTree> {
        let loader = crate::io::obj_file::ObjectFile::new(path);
        Ok(Self::new(loader.load()?, 100))
    }

    pub(crate) fn new(
        objects: Vec<Rc<RefCell<dyn RayTracable>>>,
        max_primitives_in_node: usize,
    ) -> BVHTree {
        println!("Building BVH tree...");
        let mut info: Vec<_> = objects
            .iter()
            .enumerate()
            .map(|(i, o)| BVHObjectInfo::new(i, o.borrow().bounding_box()))
            .collect();

        let mut root = BVHTree {
            data: objects,
            nodes: vec![],
            max_primitives_in_node,
            root: None,
        };

        let mut objects = vec![];
        root.root = root.recursive_build(
            &mut info,
            max_primitives_in_node,
            &mut objects,
            0,
            root.data.len(),
        );
        root.data = objects;
        println!("BVH tree built. Nodes count: {}", root.nodes.len());
        root
    }

    fn recursive_build(
        &mut self,
        info: &mut Vec<BVHObjectInfo>,
        max_primitives_in_node: usize,
        objects: &mut Vec<Rc<RefCell<dyn RayTracable>>>,
        start: usize,
        end: usize,
    ) -> Option<usize> {
        if start >= end {
            return None;
        }
        let n = end - start;
        let bound = info[start + 1..end]
            .iter()
            .fold(info[start].bounding_box, |acc, i| {
                acc.union(&i.bounding_box)
            });
        if n == 1 {
            let index = self.nodes.len();
            let node = BVHNode::new(index, bound, objects.len(), objects.len() + n);
            for i in start..end {
                objects.push(self.data[info[i].index].clone());
            }
            self.nodes.push(node);
            Some(index)
        } else {
            let axis = bound.longest_axis();
            let centroid_box = info[start..end]
                .iter()
                .fold(AlighnedBox::default(), |acc, i| acc.union_point(i.position));
            let mut mid = start + n / 2;
            if centroid_box.min[axis] == centroid_box.max[axis] {
                let index = self.nodes.len();
                let node = BVHNode::new(index, bound, objects.len(), objects.len() + n);
                for i in start..end {
                    objects.push(self.data[info[i].index].clone());
                }
                self.nodes.push(node);
                return Some(index);
            } else {
                if n <= 4 {
                    info.as_mut_slice()[start..end]
                        .sort_by(|a, b| a.position[axis].partial_cmp(&b.position[axis]).unwrap());
                } else {
                    const N: usize = 12;
                    #[derive(Copy, Clone)]
                    struct Bucket {
                        count: usize,
                        bound: AlighnedBox,
                    }
                    let mut buckets: [Bucket; N] = [Bucket {
                        count: 0,
                        bound: AlighnedBox::default(),
                    }; N];

                    for info in info[start..end].iter() {
                        let bucket_index =
                            calculate_index(&centroid_box, info.position, axis, N - 1);
                        let bucket = &mut buckets[bucket_index];
                        bucket.count += 1;
                        bucket.bound = bucket.bound.union(&info.bounding_box);
                    }

                    let mut cost = [0.0; N - 1];
                    cost.iter_mut().enumerate().for_each(|(i, c)| {
                        let mut b0 = AlighnedBox::default();
                        let mut b1 = AlighnedBox::default();
                        let mut count = 0;
                        let mut count1 = 0;
                        for j in 0..=i {
                            b0 = b0.union(&buckets[j].bound);
                            count += buckets[j].count;
                        }
                        for j in i + 1..N {
                            b1 = b1.union(&buckets[j].bound);
                            count1 += buckets[j].count;
                        }
                        *c = (b0.surface_area() * (count as f64)
                            + b1.surface_area() * (count1 as f64))
                            / bound.surface_area()
                            + 0.125;
                    });
                    let (min_bucket, min_cost) = cost
                        .into_iter()
                        .enumerate()
                        .min_by(|&(_, c0), (_, c1)| {
                            c0.partial_cmp(c1).expect("NaN value is not expected")
                        })
                        .unwrap();
                    let leaf_cost = n as f64;
                    if n > self.max_primitives_in_node || min_cost < leaf_cost {
                        info.as_mut_slice()[start..end].sort_by(|a, b| {
                            let index: usize =
                                calculate_index(&centroid_box, a.position, axis, N - 1);
                            let result1 = index <= min_bucket;
                            let index2 = calculate_index(&centroid_box, b.position, axis, N - 1);
                            let result2 = index2 <= min_bucket;
                            result1.cmp(&result2)
                        });
                        mid = info.as_slice()[start..end].partition_point(|a| {
                            calculate_index(&centroid_box, a.position, axis, N - 1) > min_bucket
                        }) + start;
                    } else {
                        let index = self.nodes.len();
                        let node = BVHNode::new(index, bound, objects.len(), objects.len() + n);
                        for i in start..end {
                            objects.push(self.data[info[i].index].clone());
                        }
                        self.nodes.push(node);
                        return Some(index);
                    }
                }
            }
            let left = self.recursive_build(info, max_primitives_in_node, objects, start, mid);
            let right = self.recursive_build(info, max_primitives_in_node, objects, mid, end);
            let index = self.nodes.len();
            let node = BVHNode::from_childrens(
                &self.nodes[left.unwrap()],
                &self.nodes[right.unwrap()],
                axis,
                index,
            );
            self.nodes.push(node);
            return Some(index);

            fn calculate_index(
                centroid: &AlighnedBox,
                point: Point,
                axis: Axis,
                max_value: usize,
            ) -> usize {
                let index =
                    ((centroid.offset(point)[axis] * max_value as f64) as usize).min(max_value);
                index as usize
            }
        }
    }
}

impl BVHObjectInfo {
    fn new(index: usize, bounding_box: AlighnedBox) -> BVHObjectInfo {
        BVHObjectInfo {
            index,
            bounding_box,
            position: bounding_box.center(),
        }
    }
}

impl BVHNode {
    fn new(index: usize, bounding_box: AlighnedBox, start: usize, end: usize) -> BVHNode {
        BVHNode {
            index,
            bounding_box,
            left: None,
            right: None,
            split_axis: None,
            start,
            end,
        }
    }

    fn from_childrens(left: &BVHNode, right: &BVHNode, axis: Axis, index: usize) -> BVHNode {
        BVHNode {
            index,
            bounding_box: left.bounding_box.union(&right.bounding_box),
            left: Some(left.index),
            right: Some(right.index),
            split_axis: Some(axis),
            start: 0,
            end: 0,
        }
    }
}
