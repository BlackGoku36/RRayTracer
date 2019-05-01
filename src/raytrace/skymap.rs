extern crate image;

use super::ray::Ray;
use super::vec::Vec3;
use std::fs::File;
use std::io::BufReader;
use std::f32::consts::PI;

pub struct IBLSkyMap{
    hdr_image: Vec<image::Rgb<f32>>,
    height: usize,
    longitude_offset: f32,
}

impl IBLSkyMap{
    pub fn new(path: &str, longitude_offset: f32) -> Self{
        let image_file = File::open(path).unwrap();
        let decoder = image::hdr::HDRDecoder::new(BufReader::new(image_file)).unwrap();
        let height = decoder.metadata().height as usize;
        let hdr_image = decoder.read_image_hdr().unwrap();
        IBLSkyMap{
            hdr_image,
            height,
            longitude_offset
        }
    }
}
pub fn radiance(map: &IBLSkyMap, r: Ray) -> Vec3{
    let dir = r.direction.normalize();
    let theta = (dir.y()).acos();
    let phi = dir.z().atan2(dir.x());
    let u = ((phi + PI + map.longitude_offset) / (2.0 * PI)) % 1.0;
    let v = (theta/PI) % 1.0;
    let height = map.height;
    let width = map.height * 2;
    let all = width * height;
    let x = (width as f32 * u).floor() as usize;
    let y = (height as f32 * v).floor() as usize;
    let index = y * width + x ;
    let col = map.hdr_image[index % all];
    Vec3::new(
        col.data[0] as f32, 
        col.data[1] as f32, 
        col.data[2] as f32,
    )
}