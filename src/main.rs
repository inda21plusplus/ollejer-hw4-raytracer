
#![allow(dead_code)]
mod ray;

use ray::Ray;
use macaw::Vec3;


fn ray_color(ray: &Ray) -> Vec3 {
    let sphere_center = Vec3::new(0.0, 0.0,-1.0);
    let t = hit_sphere(sphere_center, 0.5, ray);
    if t > 0.0 {
        let n = ray.at(t) - sphere_center;
        let n = n.normalize();

        let color = Vec3::new(n.x + 1., n.y + 1., n.z + 1.);
        return 0.5*color
    }

    let unit_direction = ray.clone().direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let color1 = Vec3::new(1.0, 1.0, 1.0);
    let color2 = Vec3::new(0.5, 0.7, 1.0);
    (1.0 - t) * color1 + t * color2
}

fn write_color(pixel_color: Vec3) {
    println!("{} {} {}", 
        (pixel_color.x * 255.999) as u8, 
        (pixel_color.y * 255.999) as u8, 
        (pixel_color.z * 255.999) as u8
    );
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - center; 
    let ray_dir = ray.direction();

    let a = ray_dir.length_squared();
    let half_b = oc.dot(ray_dir);
    let c = oc.length_squared() - radius*radius;
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

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let r = Ray::new(origin, bottom_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
    
    

}
