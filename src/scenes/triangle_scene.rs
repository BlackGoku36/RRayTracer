use crate::raytrace::{
    hitable_list::HitableList,
    material::{DiffuseLight, Lambertian},
    triangle::Triangle,
    texture::ConstantTexture,
    vec::Vec3,
};
use std::sync::Arc;

pub fn triangle_scene() -> HitableList {
    let mut world = HitableList::new(8);
    world.add(
        Box::new(Triangle::new(
            Vec3::new(-1.0, -1.0, -5.0), 
            Vec3::new(1.0, -1.0, -5.0), 
            Vec3::new(0.0, 1.0, -5.0), 
            Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.73, 0.73, 0.73,
            ))))),
        ))
    );
    world
}