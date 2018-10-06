use rand;

use rand::Rng;

use vector::Vector3;
use color::Color;

#[derive(Debug)]
pub struct PointLight {
    pub point: Vector3,
    pub color: Color,
    pub size: f64
}


impl PointLight {
    pub fn sample_point(&self) -> Vector3 {
        let mut rng = rand::thread_rng();

        let offset = Vector3::new(
            rng.gen_range(-self.size, self.size),
            rng.gen_range(-self.size, self.size),
            rng.gen_range(-self.size, self.size)
        );

        self.point + offset
    }
}