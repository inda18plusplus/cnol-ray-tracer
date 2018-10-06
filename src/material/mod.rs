mod emission;
mod reflection;
mod transparency;

use self::emission::Emission;
use self::reflection::Reflection;
use self::transparency::Transparency;

use color::Color;

#[derive(Debug)]
pub struct Material {
    pub color: Color,
    pub roughness: f64
}


impl Material {
    pub fn new(color: Color, roughness: f64) -> Material {
        Material {
            color,
            roughness
        }
    }
}
