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
    pub emission: Option<Emission>,
    pub reflection: Option<Reflection>,
    pub transparency: Option<Transparency>
}


impl Material {
    pub fn diffuse(color: Color) -> Material {
        Material {
            color,
            emission: None,
            reflection: None,
            transparency: None,
        }
    }

    pub fn emissive(color: Color, strength: f64) -> Material {
        Material {
            color,
            emission: Some(Emission {strength}),
            reflection: None,
            transparency: None,
        }
    }
}
