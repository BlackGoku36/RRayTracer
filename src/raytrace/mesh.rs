extern crate tobj;

// use super::aabb::AABB;
use super::hitable::Hitable;
use super::material::*;
use super::matrix::Matrix44;
// use super::ray::Ray;
use super::triangle::Triangle;
use super::vec::Vec3;
use std::path::Path;

use std::sync::Arc;

pub fn hitable_mesh(path: &Path, matrix: Matrix44, material: Arc<dyn Material>) -> Vec<Box<dyn Hitable>> {
    let obj = tobj::load_obj(path);
    let (models, _mtls) = obj.unwrap();
    let mut world: Vec<Box<dyn Hitable>> = vec![];

    for m in models.iter() {
        let mesh = &m.mesh;
        for f in 0..mesh.indices.len() / 3 {
            let i0 = mesh.indices[3 * f] as usize;
            let i1 = mesh.indices[3 * f + 1] as usize;
            let i2 = mesh.indices[3 * f + 2] as usize;
            let v0 = Vec3::new(
                mesh.positions[i0 * 3],
                mesh.positions[i0 * 3 + 1],
                mesh.positions[i0 * 3 + 2],
            );
            let v1 = Vec3::new(
                mesh.positions[i1 * 3],
                mesh.positions[i1 * 3 + 1],
                mesh.positions[i1 * 3 + 2],
            );
            let v2 = Vec3::new(
                mesh.positions[i2 * 3],
                mesh.positions[i2 * 3 + 1],
                mesh.positions[i2 * 3 + 2],
            );

            let tri: Triangle;
            if !mesh.normals.is_empty() {
                let normal = Vec3::new(
                    mesh.normals[i0 * 3],
                    mesh.normals[i0 * 3 + 1],
                    mesh.normals[i0 * 3 + 2],
                );
                tri = Triangle::new_normal(
                    matrix * v0,
                    matrix * v1,
                    matrix * v2,
                    normal,
                    Arc::clone(&material),
                )
            } else {
                tri = Triangle::new(matrix * v0, matrix * v1, matrix * v2, Arc::clone(&material));
            }

            world.push(Box::new(tri));
        }
    }
    world
}
