use image::Rgba;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            r, g, b
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn apply_brightness(mut self, brightness: f64) -> Color {
        self.r *= brightness;
        self.g *= brightness;
        self.b *= brightness;

        self
    }

    pub fn add(mut self, other: Color) -> Color {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;

        self
    }

    pub fn multiply(mut self, other: Color) -> Color {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;

        self
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [
            float_to_byte_color(self.r),
            float_to_byte_color(self.g),
            float_to_byte_color(self.b),
            255
        ]
    }
}

fn float_to_byte_color(float: f64) -> u8 {
    if float < 0.0 {
        0
    } else if float > 1.0 {
        255
    } else {
        (float * 255.0).floor() as u8
    }
}

impl Into<Rgba<u8>> for Color {
    fn into(self) -> Rgba<u8> {
        Rgba {
            data: self.into()
        }
    }
}