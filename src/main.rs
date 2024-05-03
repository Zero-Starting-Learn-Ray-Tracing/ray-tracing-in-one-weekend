mod ray;
mod camera;
mod hitable;
mod material;
mod sphere;

use std::f32;
use std::io::{ stderr, Write };
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::hitable::{ Hitable, HitableList };
use crate::material::{ Lambertian, Metal, Dielectric };
use crate::sphere::Sphere;

fn random_scene() -> HitableList {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world = HitableList::default();
    world.push(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(Vector3::new(0.5, 0.5, 0.5))));
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f32>();
            let center = Vector3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - origin).magnitude() > 0.9 {
                if choose_material < 0.8 { // diffuse
                    world.push(
                        Sphere::new(center, 0.2,
                                    Lambertian::new(Vector3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>()))));
                } else if choose_material < 0.95 { // metal
                    world.push(
                        Sphere::new(center, 0.2,
                                    Metal::new(Vector3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())), 0.5 * rng.gen::<f32>())));
                } else { // glass
                    world.push( Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }
    world.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    world.push(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(Vector3::new(0.4, 0.2, 0.1))));
    world.push(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)));
    world
}

fn color(ray: &Ray, world: &HitableList, depth: u32) -> Vector3<f32> {
    if let Some(hit) = world.hit(ray, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
                return attenuation.zip_map(&color(&scattered, &world, depth+1), |l, r| l * r);
            }
        }
        Vector3::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction = ray.d.normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let samples: u32 = 32;
    let mut rng = rand::thread_rng();

    // camera
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = 1080;

    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
        aperture,
        focus_dist
    );

    // world
    let world = random_scene();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f32 + rng.gen::<f32>()) / IMAGE_WIDTH as f32;
                let v = (j as f32 + rng.gen::<f32>()) / IMAGE_HEIGHT as f32;
                let ray = camera.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= samples as f32;
            for c in col.iter_mut() { *c = c.sqrt(); }
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\ndone!");
}
