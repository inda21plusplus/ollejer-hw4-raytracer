use macaw::Vec3;
use rand::Rng;

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    // clamps x to range [min, max]

    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}

pub fn write_pixel_color(pixel_color: Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / (samples_per_pixel as f32);
    let (r, g, b) = (
        (pixel_color.x * scale).sqrt(),
        (pixel_color.y * scale).sqrt(),
        (pixel_color.z * scale).sqrt(),
    );

    println!(
        "{} {} {}",
        ((256_f32 * clamp(r, 0.0, 0.999)) as u16),
        ((256_f32 * clamp(g, 0.0, 0.999)) as u16),
        ((256_f32 * clamp(b, 0.0, 0.999)) as u16),
    );
}

pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

pub fn random_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_vec() -> Vec3 {
    Vec3::new(random_f32(), random_f32(), random_f32())
}

pub fn random_vec_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

pub fn random_vec_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
