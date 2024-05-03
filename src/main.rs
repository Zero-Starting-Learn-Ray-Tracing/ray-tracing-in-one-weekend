mod ray;
mod camera;
mod hitable;
mod sphere;

use std::f32;
use std::io::{ stderr, Write };
use nalgebra::Vector3;
use crate::ray::Ray;
use crate::camera::Camera;
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
    // camera
    let camera = Camera::new();
    eprint!("Camera: {:#?}", camera);

    // world
    let world = HitableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0))
    ]);

    println!("P3\n{} {}\n255", Camera::IMAGE_WIDTH, Camera::IMAGE_HEIGHT);
    for j in (0..Camera::IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", Camera::IMAGE_HEIGHT - j);
        stderr().flush().unwrap();
        for i in 0..Camera::IMAGE_WIDTH {
            let pixel_center =
                camera.pixel00_loc
                + (i as f32 * camera.pixel_delta_u)
                + (j as f32 * camera.pixel_delta_v);
            let ray_direction = pixel_center - camera.center;
            let ray = Ray::new(camera.center, ray_direction);
            let col = color(&ray, &world);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\ndone!");
}
