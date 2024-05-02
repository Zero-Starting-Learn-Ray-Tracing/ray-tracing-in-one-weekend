mod ray;

use std::io::{ stderr, Write };
use nalgebra::Vector3;
use crate::ray::Ray;

fn hit_sphere(ray: &Ray, center: &Vector3<f32>, radius: f32) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    discriminant > 0.0
}

fn color(ray: &Ray) -> Vector3<f32> {
    if hit_sphere(ray, &Vector3::new(0.0, 0.0, -1.0), 0.8) {
        Vector3::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
    
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    const IMAGE_WIDTH: u32 = 960;
    const IMAGE_HEIGHT: u32 = 540;

    let lower_left_corner = Vector3::new(-8.0, -4.5, -1.0);
    let horizontal = Vector3::new(16.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 9.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / IMAGE_WIDTH as f32;
            let v = j as f32 / IMAGE_HEIGHT as f32;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical
            );
            let col = color(&ray);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\ndone!");
}
