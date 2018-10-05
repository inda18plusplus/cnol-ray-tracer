mod sphere;
pub use self::sphere::Sphere;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere)
}