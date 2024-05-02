mod ray;
mod hitable;
mod sphere;

use std::f32;
use std::io::{ stderr, Write };
use nalgebra::Vector3;
use crate::ray::Ray;
use crate::hitable::{ Hitable, HitableList };
use crate::sphere::Sphere;

fn color(ray: &Ray, world: &HitableList) -> Vector3<f32> {
    if let Some(hit) = world.hit(ray, 0.0, f32::MAX) {
        0.5 * hit.normal.add_scalar(1.0)
    } else {
        let unit_direction = ray.d.normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 960;
    const IMAGE_HEIGHT: u32 = if IMAGE_WIDTH as f32 / ASPECT_RATIO < 1.0 {
        1
    } else {
        (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32
    };

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);
    let camera_center = Vector3::new(0.0, 0.0, 0.0);
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (IMAGE_WIDTH as f32);
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

    let viewport_upper_left =
        camera_center
        - Vector3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // world
    let world = HitableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0))
    ]);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc
                + (i as f32 * pixel_delta_u)
                + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let col = color(&ray, &world);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\ndone!");
}
