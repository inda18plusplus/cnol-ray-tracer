mod sphere;
mod plane;

pub use self::sphere::Sphere;
pub use self::plane::Plane;

use ray::{
    Ray,
    EntryExit,
    Intersection
};

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),

    Intersection(Box<Shape>, Box<Shape>),
    Difference(Box<Shape>, Box<Shape>),
}


impl Shape {
    /// Return the first entry and exit pair of intersections
    pub fn intersection(&self, ray: &Ray) -> EntryExit {
        match self {
            Shape::Sphere(sphere) => sphere.intersection(ray),
            Shape::Plane(plane) => plane.intersection(ray),

            Shape::Intersection(a, b) => intersection(ray, a, b),
            Shape::Difference(a, b) => difference(ray, a, b),
        }
    }
}


fn intersection(ray: &Ray, a: &Shape, b: &Shape) -> EntryExit {
    let (a_entry, a_exit) = a.intersection(ray);
    let (b_entry, b_exit) = b.intersection(ray);

    if a_entry.is_some() && b_entry.is_some() {
        let entry = ray_last(ray, a_entry.unwrap(), b_entry.unwrap());
        let exit = ray_first(ray, a_exit.unwrap(), b_exit.unwrap());

        (Some(entry), Some(exit))
    } else {
        (None, None)
    }
}

fn difference(ray: &Ray, a: &Shape, b: &Shape) -> EntryExit {
    if let (Some(a_entry), Some(a_exit)) = a.intersection(ray) {
        if let (Some(b_entry), Some(b_exit)) = b.intersection(ray) {
            let a_entry_distance = ray.distance(&a_entry);
            let b_entry_distance = ray.distance(&b_entry);

            if b_entry_distance < a_entry_distance {
                let a_exit_distance = ray.distance(&a_exit);
                let b_exit_distance = ray.distance(&b_exit);

                if b_exit_distance < a_exit_distance {
                    (Some(b_exit.invert()), Some(a_exit))
                } else {
                    (None, None)
                }
            } else {
                (Some(a_entry), Some(b_entry.invert()))
            }
        } else {
            (Some(a_entry), Some(a_exit))
        }
    } else {
        (None, None)
    }
}

/// Return the intersection which was intersected first
fn ray_first(ray: &Ray, a: Intersection, b: Intersection) -> Intersection {
    let a_distance = ray.distance(&a);
    let b_distance = ray.distance(&b);

    if a_distance < b_distance {a} else {b}
}

/// Return the intersection which was intersected last
fn ray_last(ray: &Ray, a: Intersection, b: Intersection) -> Intersection {
    let a_distance = ray.distance(&a);
    let b_distance = ray.distance(&b);

    if a_distance > b_distance {a} else {b}
}
