use nalgebra::Vector3;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    pub center: Vector3<f32>,
    pub pixel00_loc: Vector3<f32>,// location of pixel (0, 0)
    pub pixel_delta_u: Vector3<f32>,
    pub pixel_delta_v: Vector3<f32>
}

impl Camera {
    pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
    pub const IMAGE_WIDTH: u32 = 960;
    pub const IMAGE_HEIGHT: u32 = if Self::IMAGE_WIDTH as f32 / Self::ASPECT_RATIO < 1.0 {
        1
    } else {
        (Self::IMAGE_WIDTH as f32 / Self::ASPECT_RATIO) as u32
    };

    pub fn new() -> Self {
        let center = Vector3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (Self::IMAGE_WIDTH as f32 / Self::IMAGE_HEIGHT as f32);
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, viewport_height, 0.0);
        let pixel_delta_u = viewport_u / Self::IMAGE_WIDTH as f32;
        let pixel_delta_v = viewport_v / Self::IMAGE_HEIGHT as f32;

        let viewport_upper_left =
            center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        return Camera {
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v
        };
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let pixel_center =
            self.pixel00_loc
            + (u as f32 * self.pixel_delta_u)
            + (v as f32 * self.pixel_delta_v);
        let ray_direction = pixel_center - self.center;
        
        Ray::new(self.center, ray_direction)
    }
}
