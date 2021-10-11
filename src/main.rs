mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod util;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use macaw::Vec3;
use material::scatter;
use ray::Ray;
use sphere::Sphere;
use util::{random_f32, random_unit_vector, random_vec_in_hemisphere, write_pixel_color};

use crate::material::Material;

fn ray_color(ray: &Ray, world: &HittableList, depth: u8) -> Vec3 {
    let mut hit_rec = HitRecord::default();

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.hit(ray, 0.001, f32::MAX) {
        let mut scattered_ray = Ray::default();
        let mut attenuation = Vec3::default();
        if scatter(
            &hit_rec.material,
            ray,
            &hit_rec,
            &mut attenuation,
            &mut scattered_ray,
        ) {
            return attenuation * ray_color(&scattered_ray, world, depth - 1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction().normalize();
    let transition_variable = 0.5 * (unit_direction.y + 1.0);
    let color1 = Vec3::new(1.0, 1.0, 1.0);
    let color2 = Vec3::new(0.5, 0.7, 1.0);

    // Using formula: Blended value = (1-t)*startvalue + t*endvalue
    (1.0 - transition_variable) * color1 + transition_variable * color2
}

fn main() {
    // Image

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    const MAX_VALUE: u8 = 255;

    const SAMPLES_PER_PIXEL: u32 = 100;

    const MAX_DEPTH: u8 = 50;

    // World
    let material_ground = Material::Matte {
        albedo: Vec3::new(0.0, 0.4, 0.0),
    };
    let material_center = Material::Matte {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    };

    let material_left = Material::Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let material_right = Material::Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    let mut world = HittableList::new();
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    // ground
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.list.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let camera = Camera::new();

    // Render

    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE);

    for pixel_row in (0..IMAGE_HEIGHT).rev() {
        for pixel_column in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (pixel_column as f32 + random_f32()) / (IMAGE_WIDTH) as f32;
                let v = (pixel_row as f32 + random_f32()) / (IMAGE_HEIGHT) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }
            write_pixel_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
