

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
}

impl Hittable for HittableList {
    

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, mut rec: &mut  HitRecord ) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;


        for object in &self.list {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                
                rec.point = temp_rec.point;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
                rec.front_face = temp_rec.front_face;

            }
        }
        hit_anything
    }
}