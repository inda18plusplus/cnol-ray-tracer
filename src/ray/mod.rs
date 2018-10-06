
mod intersection;

pub use self::intersection::{
    Intersection,
    EntryExit
};

use vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}


impl Ray {
    // https://math.stackexchange.com/questions/13261/how-to-get-a-reflection-vector#13266
    pub fn bounce(ray: &Ray, intersection: Intersection) -> Ray {
        let normal = intersection.normal;

        let reflection = ray.direction - 2.0 * Vector3::dot(ray.direction, normal) * normal;

        Ray {
            origin: intersection.point,
            direction: reflection.normal(),
        }
    }
}


