use std::fmt;


struct Vector3 {
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

#[derive(Debug)]
struct Intersection {
    pub point: Vector3,
    pub normal: Vector3
}

#[derive(Debug)]
struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

trait Traceable {
    fn intersections(&self, ray: Ray) -> Vec<Intersection>;
}

#[derive(Debug)]
struct Sphere {
    center: Vector3,
    radius: f64
}

impl Traceable for Sphere {
    fn intersections(&self, ray: Ray) -> Vec<Intersection> {
        Vec::new()
    }
}

fn main() {
    println!("Hello, world!");

    let sphere = Sphere {
        center: Vector3::new(0.0, 0.0, 0.0),
        radius: 1.0
    };

    println!("{:?}", sphere);
}
