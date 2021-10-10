
#![allow(dead_code)]
mod ray;
mod sphere;
mod hittable;
mod hittable_list;

use ray::Ray;
use macaw::Vec3;
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use sphere::Sphere;


fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    let mut hit_rec = HitRecord::default();
    if world.hit(ray, 0.0, f32::MAX, &mut hit_rec) {
        return 0.5 * (hit_rec.normal + Vec3::new(0.5, 1.0, 1.0)); // Change x value to 1.0
    }

    let unit_direction = ray.direction().normalize();
    let transition_variable = 0.5 * (unit_direction.y + 1.0);
    let color1 = Vec3::new(1.0, 1.0, 1.0);
    let color2 = Vec3::new(0.5, 0.7, 1.0);

    // Using formula: Blended value = (1-t)*startvalue + t*endvalue
    (1.0 - transition_variable) * color1 + transition_variable * color2
} 

fn write_pixel_color(pixel_color: Vec3) {
    println!("{} {} {}", 
        (pixel_color.x * 255.999) as u8, 
        (pixel_color.y * 255.999) as u8, 
        (pixel_color.z * 255.999) as u8
    );
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let distbetween_origin_center = ray.origin() - center; 
    let ray_dir = ray.direction();

    let a = ray_dir.length_squared();
    let half_b =distbetween_origin_center.dot(ray_dir);
    let c = distbetween_origin_center.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0 {
        -1.0 
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}



fn main() {

    // Image 
    
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    const MAX_VALUE: u8 = 255; 

    // World
    // small sphere  
    let mut world = HittableList::new();
    world.list.push(
        Box::new(
            Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5
            )
        )
    );
    // ground
    world.list.push(
        Box::new(
            Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
            )
        )
    );



    // Camera 
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length:f32 = 1.0;

    let origin = Vec3::new(0.0, 0.0,0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let bottom_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);


    // Render 

    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE);

    for pixel_row in (0..IMAGE_HEIGHT).rev() {
        for pixel_column in 0..IMAGE_WIDTH {
            let u = pixel_column as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = pixel_row as f32 / (IMAGE_HEIGHT - 1) as f32;

            let ray = Ray::new(origin, bottom_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(&ray, &world);
            write_pixel_color(pixel_color);
        }
    }
    
    

}
