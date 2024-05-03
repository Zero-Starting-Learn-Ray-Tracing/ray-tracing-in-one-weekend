use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::hitable::HitRecord;

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

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)>;
}

pub struct Lambertian {
    albedo: Vector3<f32>
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Self { Lambertian { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Vector3<f32>
}

impl Metal {
    pub fn new(albedo: Vector3<f32>) -> Self { Metal { albedo } }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let reflected = reflect(&ray.d.normalize(), &hit.normal);
        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray::new(hit.p, reflected);
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
