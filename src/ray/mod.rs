use rand;
use rand::Rng;


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
    pub fn bounce(&self, intersection: Intersection) -> Ray {
        let normal = intersection.normal;

        let reflection = self.direction - 2.0 * Vector3::dot(self.direction, normal) * normal;

        Ray {
            origin: intersection.point,
            direction: reflection.normal(),
        }
    }


    pub fn scatter(&self, intersection: Intersection, roughness: f64) -> Ray {
        let mut rng = rand::thread_rng();

        let right = Vector3::cross(self.direction, intersection.normal);
        let up = Vector3::cross(right, self.direction);

        let distance_x = if roughness > 0.0 { rng.gen_range(-roughness, roughness) } else {0.0};
        let distance_y = if roughness > 0.0 { rng.gen_range(-roughness, roughness) } else {0.0};

        let mut bounce = self.bounce(intersection);

        bounce.direction = (
            bounce.direction +
            right * distance_x +
            up * distance_y
        ).normal();

        bounce
    }

    /// The distance to intersection
    pub fn distance(&self, intersection: &Intersection) -> f64 {
        Vector3::distance(self.origin, intersection.point)
    }
}


