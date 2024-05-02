use nalgebra::Vector3;

pub struct Ray {
    o: Vector3<f32>,
    d: Vector3<f32>,
}

impl Ray {
    pub fn new(o: Vector3<f32>, d: Vector3<f32>) -> Self {
        Ray { o, d }
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.o
    }
    pub fn direction(&self) -> Vector3<f32> {
        self.d
    }
    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.o + t * self.d
    }
}
