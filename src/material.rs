use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util::{random_unit_vector, random_vec_in_unit_sphere, vec_near_zero, vec_reflect};
use macaw::Vec3;

#[derive(Clone, PartialEq)]
pub enum Material {
    Metal { albedo: Vec3, fuzz: f32 },
    Matte { albedo: Vec3 },
}

impl Default for Material {
    fn default() -> Self {
        Material::Matte {
            albedo: Vec3::default(),
        }
    }
}

pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    hit_rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered_ray: &mut Ray,
) -> bool {
    match material {
        &Material::Matte { albedo } => {
            let mut scatter_direction = hit_rec.normal + random_unit_vector();

            if vec_near_zero(scatter_direction) {
                scatter_direction = hit_rec.normal;
            }

            *scattered_ray = Ray::new(hit_rec.point, scatter_direction);
            *attenuation = albedo;
            true
        }
        &Material::Metal { albedo, fuzz } => {
            let reflected_direction = vec_reflect(&ray_in.direction().normalize(), &hit_rec.normal);
            *scattered_ray = Ray::new(
                hit_rec.point,
                reflected_direction + fuzz * random_vec_in_unit_sphere(),
            );
            *attenuation = albedo;
            let scattered_direction = scattered_ray.direction();
            scattered_direction.dot(hit_rec.normal) > 0.0
        }
    }
}
