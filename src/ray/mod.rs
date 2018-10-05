
mod traceable;
mod intersection;

pub use self::traceable::Traceable;
pub use self::intersection::Intersection;

#[derive(Debug)]
struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}


