use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::ray::Ray;

pub struct HitableList {
    pub objects: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list_size: usize) -> Self {
        HitableList {
            objects: Vec::with_capacity(list_size),
        }
    }
    pub fn add(&mut self, sphere: Box<dyn Hitable>) {
        self.objects.push(sphere);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for i in &self.objects {
            let temp_result = i.hit(r, t_min, closest_so_far);
            if let Some(rec) = temp_result {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }
        hit_anything
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        if let Some(first) = self.objects[0].bounding_box(t0, t1) {
            let mut result = first;
            for sphere in &self.objects[1..] {
                if let Some(bbox) = sphere.bounding_box(t0, t1) {
                    result = result.surrounding_box(&bbox);
                } else {
                    return None;
                }
            }
            Some(result)
        } else {
            None
        }
    }
}
