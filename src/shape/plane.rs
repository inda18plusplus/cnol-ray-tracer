use vector::Vector3;
use ray::Ray;
use ray::{
    Intersection,
    EntryExit
};

#[derive(Debug, Clone)]
pub struct Plane {
    pub origin: Vector3,
    pub normal: Vector3
}

impl Plane {
    /// Return the first entry and exit pair of intersections
    // https://www.wikiwand.com/en/Line%E2%80%93plane_intersection
    pub fn intersection(&self, ray: &Ray) -> EntryExit {
        let direction = Vector3::dot(ray.direction, self.normal);
        let parallel = direction == 0.0;

        if parallel {
            (None, None)
        } else {
            let numerator = Vector3::dot(self.origin - ray.origin, self.normal);
            let distance = numerator / direction;

            if distance > 0.0 {
                let point = ray.origin + distance * ray.direction;
                let normal = if direction < 0.0 { self.normal } else { -self.normal };

                let intersection = Intersection {
                    point,
                    normal,
                };

                (Some(intersection.clone()), Some(intersection))
            } else {
                (None, None)
            }
        }
    }
}