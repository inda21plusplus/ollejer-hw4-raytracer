use macaw::Vec3;
use rand::Rng;

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    // clamps x to range [min, max]
    
    if x < min {return min};
    if x > max {return max};
    return x;
}


pub fn write_pixel_color(pixel_color: Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / (samples_per_pixel as f32);
    let (r, g, b) = (pixel_color.x*scale, pixel_color.y*scale, pixel_color.z*scale);

 
    println!("{} {} {}", 
        ((256 as f32 * clamp(r, 0.0, 0.999)) as u16),
        ((256 as f32 * clamp(g, 0.0, 0.999)) as u16),
        ((256 as f32 * clamp(b, 0.0, 0.999)) as u16),    
    );
    
}

pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

pub fn random_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max)
}
