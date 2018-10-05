mod sphere;
pub use self::sphere::Sphere;
use ray::Ray;
use ray::Intersection;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere)
}


impl Shape {
    /// Return the first entry and exit pair of intersections
    pub fn intersection(&self, ray: &Ray) -> (Option<Intersection>, Option<Intersection>) {
        match self {
            Shape::Sphere(sphere) => sphere.intersection(ray)
        }
    }
}