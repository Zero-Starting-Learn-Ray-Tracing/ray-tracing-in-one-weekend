use nalgebra::Vector3;

pub struct Ray {
    pub o: Vector3<f32>,
    pub d: Vector3<f32>,
}

impl Ray {
    pub fn new(o: Vector3<f32>, d: Vector3<f32>) -> Self {
        Ray { o, d }
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.o + t * self.d
    }
}
