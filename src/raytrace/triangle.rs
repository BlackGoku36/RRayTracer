use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec::Vec3;

use std::sync::Arc;

pub struct Triangle{
    v0: Vec3, v1: Vec3, v2: Vec3,
    normal: Vec3,
    material: Arc<Material>,
}

impl Triangle{
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Arc<Material>)-> Self{
        Triangle{
            v0, v1, v2,
            normal: Vec3::cross(v1-v0, v2-v0),
            material,
        }
    }
    pub fn new_normal(v0: Vec3, v1: Vec3, v2: Vec3, normal: Vec3, material: Arc<Material>)-> Self{
        Triangle{
            v0, v1, v2,
            normal,
            material,
        }
    }
}

impl Hitable for Triangle{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let v0 = self.v0;
        let edge1 = self.v1 - v0;
        let edge2 = self.v2 - v0;

        let h = Vec3::cross(r.direction, edge2);
        let a = Vec3::dot(&edge1, &h);

        if a.abs() < std::f32::EPSILON{
            return None;
        }
        
        let f = 1.0/a;
        let s = r.origin - v0;
        let u = f * Vec3::dot(&s, &h);

        if u < 0.0 || u > 1.0{
            return None;
        }

        let q = Vec3::cross(s, edge1);
        let v = f * Vec3::dot(&r.direction, &q);

        if v < 0.0 || u + v > 1.0{
            return None;
        }

        let t = f * Vec3::dot(&edge2, &q);
        if t < t_min || t > t_max{
            return None;
        }
        Some(HitRecord{
            t,
            u,
            v,
            p: r.point_at_parameter(t),
            normal: self.normal,
            material: self.material.clone(),
        })


    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(
                self.v0.x().min(self.v1.x().min(self.v2.x())), 
                self.v0.y().min(self.v1.y().min(self.v2.y())), 
                self.v0.z().min(self.v1.z().min(self.v2.z())),
            ),
            max: Vec3::new(
                self.v0.x().max(self.v1.x().max(self.v2.x())), 
                self.v0.y().max(self.v1.y().max(self.v2.y())), 
                self.v0.z().max(self.v1.z().max(self.v2.z())),
            ),
        })
    }

}