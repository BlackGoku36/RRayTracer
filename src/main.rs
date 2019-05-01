
pub mod raytrace;
pub mod scenes;

use raytrace::ray::Ray;
use raytrace::vec::Vec3;

use raytrace::camera::Camera;
use raytrace::hitable::Hitable;
use raytrace::hitable_list::HitableList;
use raytrace::vec::drand48;
use raytrace::skymap::IBLSkyMap;
use raytrace::skymap::radiance;
use rayon::prelude::*;
use indicatif::{
    ProgressBar, ProgressStyle,
};

use scenes::{
    lighted_perlin_spheres::lightted_perlin_spheres,
    cornell_box::{cornell_box, cornell_smoke},
    final_scene::final_scene,
    // checkered_texture::checkered_texture_scene,
    // default_scene::default_scene,
    perlin_spheres::perlin_spheres,
    textured_sphere::textured_spheres,
    triangle_scene::triangle_scene,
    random_spheres::random_scene,
};

fn color(r: Ray, world: &HitableList, depth: i32, map: &IBLSkyMap) -> Vec3 {
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            if depth >= 50 {
                return Vec3::new(0.0, 0.0, 0.0);
            }
            let (mut emitted, lpos) = rec.material.emitted(rec.u, rec.v, rec.p);
            let surface_normal = -rec.normal;
            let direction_to_light = (lpos-rec.p).normalize();
            let shadow_ray = Ray::new(
                rec.p+(surface_normal*0.001), 
                direction_to_light, 
                0.0
            );
            match world.hit(shadow_ray, 0.001, std::f32::MAX){
                Some(_l) => {
                    emitted = emitted;
                }
                None => {
                    emitted = Vec3::new(0.0, 0.0, 0.0);
                }
            }
            if let Some((scattered, attenuation)) = rec.material.scatter(&r, &rec) {
                emitted + attenuation * color(scattered, world, depth + 1, map)
            } else {
                emitted
            }
        }
        None => {
            radiance(map, r)
            // Vec3::new(0.0, 0.0, 0.0)
            // let unit_direction = Vec3::unit_vector(r.direction());
            // let t = 0.5 * (unit_direction.y() + 1.0);
            // return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    let nx = 1920;
    let ny = 1080;
    let ns = 100;

    let progress_bar = ProgressBar::new((nx as usize * ny as usize/64) as u64);
    progress_bar.set_prefix("Tracing some rays");
    progress_bar.set_style(ProgressStyle::default_bar()
      .template("{prefix:.white} [{elapsed_precise}] {bar:40.cyan/blue} {percent}%"));

    print!("P3\n{} {}\n255\n", nx, ny);
    let look_from: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let look_at: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    // let look_from: Vec3 = Vec3::new(278.0, 278.0, -800.0);
    // let look_at: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    // let look_from: Vec3 = Vec3::new(25.0, 3.0, 5.0);
    // let look_at: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature: f32 = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        nx as f32 / ny as f32,
        aperature,
        dist_to_focus,
        0.0,
        1.0,
    );

    let world = random_scene();
    let map = IBLSkyMap::new("assets/sunrise.hdr", 3.3);
    let rows: Vec<Vec<Vec3>> = (0..ny)
        .into_par_iter()
        .rev()
        .map(|j| {
            (0..nx)
                .into_par_iter()
                .map(|i| {
                    let mut col = Vec3::new(0.0, 0.0, 0.0);
                    for _s in 0..ns {
                        let u = (i as f32 + drand48()) / nx as f32;
                        let v = (j as f32 + drand48()) / ny as f32;
                        let r = cam.get_ray(u, v);
                        col += color(r, &world, 0, &map);
                    }
                    if i % 64 == 0{
                        progress_bar.inc(1);
                    }
                    col /= ns as f32;
                    col = Vec3::new(f32::sqrt(col[0]), f32::sqrt(col[1]), f32::sqrt(col[2]));
                    col *= 255.99;
                    col
                })
                .collect()
        })
        .collect();
        progress_bar.finish_with_message("finished");

    for r in rows {
        for col in r {
            print!("{} {} {}\n", col.r().min(255.0) as i32, col.g().min(255.0) as i32, col.b().min(255.0) as i32);
        }
    }
}
