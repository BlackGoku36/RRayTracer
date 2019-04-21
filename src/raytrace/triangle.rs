use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec::Vec3;

use std::sync::Arc;

pub struct Triangle{
    v0: Vec3, v1: Vec3, v2: Vec3,
    material: Arc<Material>,
}

impl Triangle{
    pub fn new(v0: Vec3, 
    v1: Vec3, v2: Vec3, material: Arc<Material>)-> Self{
        Triangle{
            v0, v1, v2,
            material,
        }
    }
}

impl Hitable for Triangle{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;

        let n = Vec3::cross(v0v1, v0v2);
        let denom = Vec3::dot(&n, &n);

        let ndotRayDirection = Vec3::dot(&n, &r.direction);
        if ndotRayDirection.abs() < std::f32::EPSILON{
            return None;
        }
        
        let d = Vec3::dot(&n, &self.v0);
        let t = (Vec3::dot(&n, &r.origin)+d)/ndotRayDirection;
        if t < 0.0{
            return None;
        }
        let p = r.origin + t * r.direction;

        let mut c: Vec3;

        let edge0 = self.v1 - self.v0;
        let vp0 = p - self.v0;
        c = Vec3::cross(edge0, vp0);
        if Vec3::dot(&n, &c) < 0.0{
            return None;
        }

        let edge1 = self.v2 - self.v1;
        let vp1 = p - self.v1;
        c = Vec3::cross(edge1, vp1);
        let mut u = Vec3::dot(&n, &c);
        if u < 0.0{
            return None;
        }

        let edge2 = self.v0 - self.v2;
        let vp2 = p - self.v2;
        c = Vec3::cross(edge2, vp2);
        let mut v = Vec3::dot(&n, &c);
        if v < 0.0{
            return None;
        }

        u /= denom;
        v /= denom;

        Some(HitRecord{
            t,
            u,
            v,
            p,
            normal: n,
            material: self.material.clone(),
        })

    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let maxX = max(self.v0.x(), max(self.v1.x(), self.v2.x()));
        let minX = min(self.v0.x(), min(self.v1.x(), self.v2.x()));
        let maxY = max(self.v0.y(), max(self.v1.y(), self.v2.y()));
        let minY = min(self.v0.y(), min(self.v1.y(), self.v2.y()));
        let maxZ = max(self.v0.z(), max(self.v1.z(), self.v2.z()));
        let minZ = min(self.v0.z(), min(self.v1.z(), self.v2.z()));
        Some(AABB {
            min: Vec3::new(minX, minY, minZ),
            max: Vec3::new(maxX, maxY, maxZ),
        })
    }

}
fn max(a: f32, b: f32)-> f32{
    if a > b{
        a
    }else{
        b
    }
}
fn min(a: f32, b: f32)-> f32{
    if a < b{
        a
    }else{
        b
    }
}