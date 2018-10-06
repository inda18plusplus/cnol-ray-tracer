
mod point_light;

pub use self::point_light::PointLight;
use vector::Vector3;
use color::Color;

#[derive(Debug)]
pub enum Light {
    Point(PointLight)
}

impl Light {
    /// Get a random point within the light
    pub fn sample_point(&self) -> Vector3 {
        match self {
            Light::Point(point_light) => point_light.sample_point()
        }
    }

    /// Get the color of the light
    pub fn color(&self) -> Color {
        match self {
            Light::Point(point_light) => point_light.color
        }
    }


    /// Get the brightness of this light
    pub fn brightness(&self, distance: f64) -> f64 {
        0.5 / distance
    }
}