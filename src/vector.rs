use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 {
            x, y, z
        }
    }

    pub fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vector3 {
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normal(self) -> Self {
        let length = self.length();
        self / length
    }

    pub fn dot(self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn distance(self, other: Vector3) -> f64 {
        (self - other).length()
    }

    // https://www.wikiwand.com/en/Cross_product
    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(mut self, rhs: Vector3) -> Vector3 {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;

        self
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(mut self, rhs: Vector3) -> Vector3 {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;

        self
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, mut rhs: Vector3) -> Vector3 {
        rhs.x *= self;
        rhs.y *= self;
        rhs.z *= self;

        rhs
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(mut self, rhs: f64) -> Vector3 {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;

        self
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(mut self, rhs: f64) -> Vector3 {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;

        self
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(mut self) -> Vector3 {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;

        self
    }
}


impl fmt::Display for Vector3 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Vector3 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        (self as &fmt::Display).fmt(formatter)
    }
}
