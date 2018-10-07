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
    pub fn intersection(&self, ray: &Ray) -> Vec<EntryExit> {
        let direction = Vector3::dot(ray.direction, self.normal);
        let parallel = direction == 0.0;

        if parallel || direction > 0.0 {
            Vec::new()
        } else {
            let numerator = Vector3::dot(self.origin - ray.origin, self.normal);
            let distance = numerator / direction;

            if distance > 0.0 {
                let point = ray.origin + distance * ray.direction;

                let entry = Intersection {
                    point,
                    normal: self.normal,
                    distance
                };

                use std::f64::INFINITY;

                let exit = Intersection {
                    point: ray.direction * INFINITY,
                    normal: -self.normal,
                    distance: INFINITY
                };

                vec![(entry, exit)]
            } else {
                Vec::new()
            }
        }
    }
}