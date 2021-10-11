mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod util;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use macaw::Vec3;
use ray::Ray;
use sphere::Sphere;
use util::{random_f32, write_pixel_color};

fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    let mut hit_rec = HitRecord::default();
    if world.hit(ray, 0.0, f32::MAX, &mut hit_rec) {
        return 0.4 * (hit_rec.normal + Vec3::new(0.5, 1.0, 1.0)); // Change x value to 1.0
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
    const IMAGE_WIDTH: i32 = 200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    const MAX_VALUE: u8 = 255;

    const SAMPLES_PER_PIXEL: u32 = 50;

    // World
    // small sphere
    let mut world = HittableList::new();
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    // ground
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

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
                pixel_color += ray_color(&ray, &world);
            }
            write_pixel_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
