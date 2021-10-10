use crate::ray::Ray;
use macaw::Vec3;

pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,

    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    bottom_left_corner: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0_f32;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let bottom_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,

            origin,
            horizontal,
            vertical,
            bottom_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction =
            self.bottom_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sixteen_by_nine() {
        let camera = Camera::new();
        assert_eq!(camera.aspect_ratio, 16.0 / 9.0 as f32);
    }
}
