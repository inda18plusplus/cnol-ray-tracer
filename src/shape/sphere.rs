use vector::Vector3;
use ray::Ray;
use ray::Intersection;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}


impl Sphere {
    /// Returns the entry and exit points of a ray respectively
    pub fn intersection(&self, ray: &Ray) -> (Option<Intersection>, Option<Intersection>) {
        let projection = Vector3::dot(self.center - ray.origin, ray.direction);

        // The point on the ray closest to the center of the sphere
        let closest_point_on_ray = ray.origin + projection * ray.direction;

        // TODO: Remove unnecessary square root
        let distance = Vector3::distance(closest_point_on_ray, self.center);

        // Test if inside sphere
        if distance < self.radius {
            let distance_to_intersection = (self.radius * self.radius + distance * distance).sqrt();

            let distance_to_entry = projection - distance_to_intersection;
            let distance_to_exit = projection + distance_to_intersection;

            let entry = self.intersection_along_ray(distance_to_entry, ray);
            let exit = self.intersection_along_ray(distance_to_exit, ray);

            (entry, exit)
        } else {
            (None, None)
        }
    }

    fn intersection_along_ray(&self, distance_along_ray: f64, ray: &Ray) -> Option<Intersection> {
        if distance_along_ray > 0.0 {
            let point = ray.origin + distance_along_ray * ray.direction;
            let normal = Vector3::normal(point - self.center);
            Some(Intersection { point, normal })
        } else {
            None
        }
    }
}

