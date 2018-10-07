use color::Color;

#[derive(Debug)]
pub struct Material {
    pub color: Color,
    pub roughness: f64,
    pub reflectiveness: f64
}


impl Material {
    pub fn new(color: Color, roughness: f64, reflectiveness: f64) -> Material {
        Material {
            color,
            roughness,
            reflectiveness
        }
    }
}
