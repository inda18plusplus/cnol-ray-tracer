mod sphere;
mod plane;

pub use self::sphere::Sphere;
pub use self::plane::Plane;

use ray::Ray;
use ray::EntryExit;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane)
}


impl Shape {
    /// Return the first entry and exit pair of intersections
    pub fn intersection(&self, ray: &Ray) -> EntryExit {
        match self {
            Shape::Sphere(sphere) => sphere.intersection(ray),
            Shape::Plane(plane) => plane.intersection(ray)
        }
    }
}