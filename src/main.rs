mod ray;
mod camera;
mod hitable;
mod sphere;

use std::f32;
use std::io::{ stderr, Write };
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::hitable::{ Hitable, HitableList };
use crate::sphere::Sphere;

fn random_in_unit_sphere() -> Vector3<f32> {
    // 拒绝法
    // 在单位正方体内生成单位球坐标
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0
            * Vector3::new(
                rng.gen::<f32>(),// 生成 0-1 随机数
                rng.gen::<f32>(),
                rng.gen::<f32>()
            ) - unit;
        if p.magnitude_squared() < 1.0 { return p; }
    }
}

fn color(ray: &Ray, world: &HitableList) -> Vector3<f32> {
    if let Some(hit) = world.hit(ray, 0.001, f32::MAX) {
        let temp = random_in_unit_sphere();
        let target = if temp.dot(&hit.normal) > 0.0 { temp } else { -temp };
        // let base_color = Vector3::new(0.5, 0.5, 0.5);
        let base_color = Vector3::new(0.0, 250.0 / 255.0, 154.0 / 255.0);
        base_color.component_mul(&color(&Ray::new(hit.p, target), world))
    } else {
        let unit_direction = ray.d.normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let samples: u32 = 16;
    let mut rng = rand::thread_rng();

    // camera
    let camera = Camera::new();

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
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let ray = Camera::get_ray(
                    &camera,
                    i as f32 + rng.gen::<f32>(),
                    j as f32 + rng.gen::<f32>()
                );
                col += color(&ray, &world);
            }
            col /= samples as f32;
            // for c in col.iter_mut() { *c = c.sqrt(); }
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\ndone!");
}
