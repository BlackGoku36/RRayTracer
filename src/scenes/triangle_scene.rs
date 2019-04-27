use crate::raytrace::{
    hitable_list::HitableList,
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    triangle::Triangle,
    vec::Vec3,
    mesh::hitable_mesh,
    bvh::BVHNode,
    texture::{ConstantTexture, ImageTexture, NoiseTexture},
    rectangle::{FlipNormal, XY, XZ, YZ},
    cube::{Cube, RotateY, Translate},
    matrix::Matrix44,
};
use std::sync::Arc;
use std::path::Path;

pub fn triangle_scene() -> HitableList {
    let mut world = HitableList::new(8);
    let bunny = hitable_mesh(
        Path::new("bunny.obj"),
        Matrix44::translate(200.0, 0.0, 300.0)*
        Matrix44::scale_linear(120.0) * Matrix44::rotate_y(-1.4),
        Arc::new(Dielectric::new(1.5)),
    );
    let suzanne = hitable_mesh(
        Path::new("suzanne.obj"),
        Matrix44::translate(390.0, 150.0, 300.0)*
        Matrix44::scale_linear(110.0) * Matrix44::rotate_y(10.0)*Matrix44::rotate_x(-0.2),
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.7, 0.3, 0.2,
        ))))),
    );
    let image = image::open("earth.png").expect("Can't find image").to_rgb();
    let (nx, ny) = image.dimensions();
    let pixels = image.into_raw();
    let texture = ImageTexture::new(pixels, nx, ny);
    world.add(
        Translate::new(
            Box::new(Triangle::new(
                Vec3::new(-1.0, -1.0, -5.0), 
                Vec3::new(1.0, -1.0, -5.0), 
                Vec3::new(0.0, 1.0, -5.0), 
                Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 0.1)),
            )),
            Vec3::new(200.0, 0.0, 300.0)
        )
    );
    world.add(
        BVHNode::construct(bunny, 0.0, 1.0),
    );
    world.add(
        BVHNode::construct(suzanne, 0.0, 1.0),
    );
    let red = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    )))));
    let green = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    )))));
    let white = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    let light = Arc::new(DiffuseLight::new(Box::new(ConstantTexture::new(
        Vec3::new(15.0, 15.0, 15.0),
    ))));
    world.add(FlipNormal::new(Box::new(YZ::new(
        0.0, 555.0, 0.0, 555.0, 555.0, red,
    ))));
    world.add(Box::new(YZ::new(0.0, 555.0, 0.0, 555.0, 0.0, green)));
    world.add(Box::new(XZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    world.add(FlipNormal::new(Box::new(XZ::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.2, 0.3, 0.7,
        ))))),
    ))));
    world.add(Box::new(XZ::new(0.0, 555.0, 0.0, 555.0, 0.0, Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 0.0)))));
    world.add(FlipNormal::new(Box::new(XY::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.73, 0.73, 0.73,
        ))))),
    ))));
    world
}