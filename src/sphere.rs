
use macaw::Vec3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};


pub struct Sphere {
    center: Vec3,
    radius: f32, 
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center; 
        let ray_dir = ray.direction();

        let a = ray_dir.length_squared();
        let half_b = oc.dot(ray_dir);
        let c = oc.length_squared() - self.radius*self.radius;
        
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false
        };

        let sqrt_discriminant = discriminant.sqrt();
        let root = (-half_b - sqrt_discriminant) / a;
        
        if root < t_min || root > t_max {
            let root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        
        // Hidden behavior, needs cleanup
        rec.t = root;
        rec.point = ray.at(rec.t);     
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        return true;
    }
}