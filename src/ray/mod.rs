
mod intersection;

pub use self::intersection::Intersection;

use vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}


