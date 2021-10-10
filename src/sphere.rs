
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool {
        // Using formula for sphere intersection 
        // (P−C)⋅(P−C)=r2, P(t) = A+tb
        // (P(t)−C)⋅(P(t)−C)=r2
        // (A+tb−C)⋅(A+tb−C)=r2
        // t2b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r2=0

        let origin_center_dist = ray.origin() - self.center; 
        let ray_dir = ray.direction();

        let a = ray_dir.length_squared();
        let half_b = origin_center_dist.dot(ray_dir);
        let c = origin_center_dist.length_squared() - self.radius*self.radius;
        
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
        hit_rec.t = root;
        hit_rec.point = ray.at(hit_rec.t);     
        let outward_normal = (hit_rec.point - self.center) / self.radius;
        hit_rec.set_face_normal(ray, outward_normal);

        return true;
    }
}