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
    pub fn first_intersection(&self, ray: &Ray) -> Option<EntryExit> {
        let intersections = match self {
            Shape::Sphere(sphere) => sphere.intersection(ray),
            Shape::Plane(plane) => plane.intersection(ray),

            Shape::Intersection(a, b) => intersection(ray, a, b),
            Shape::Difference(a, b) => difference(ray, a, b),
        };

        if intersections.len() == 0 {
            None
        } else {
            intersections.first().map(|a| a.clone())
        }
    }

    pub fn all_intersections(&self, ray: &Ray) -> Vec<EntryExit> {
        match self {
            Shape::Sphere(sphere) => sphere.intersection(ray),
            Shape::Plane(plane) => plane.intersection(ray),

            Shape::Intersection(a, b) => intersection(ray, a, b),
            Shape::Difference(a, b) => difference(ray, a, b),
        }
    }
}


fn intersection(ray: &Ray, a: &Shape, b: &Shape) -> Vec<EntryExit> {
    let regions = get_regions(ray, a, b);
    regions.into_iter()
        .filter(|region| region.a && region.b)
        .map(|region| (region.start, region.end))
        .collect()
}

fn difference(ray: &Ray, a: &Shape, b: &Shape) -> Vec<EntryExit> {
    let regions = get_regions(ray, a, b);

    regions.into_iter()
        .filter(|region| region.a && !region.b)
        .map(|region| (region.start, region.end))
        .collect()
}


#[derive(Clone, Debug)]
struct Region {
    start: Intersection,
    end: Intersection,
    a: bool,
    b: bool
}


fn get_regions(ray: &Ray, a: &Shape, b: &Shape) -> Vec<Region> {
    enum Owner {A, B}

    let a_intersections = a.all_intersections(ray);
    let b_intersections = b.all_intersections(ray);

    let mut intersections = a_intersections.into_iter().flat_map(|(entry, exit)|{
        vec![(entry, Owner::A), (exit.inverse(), Owner::A)]
    }).chain(b_intersections.into_iter().flat_map(|(entry, exit)|{
        vec![(entry, Owner::B), (exit.inverse(), Owner::B)]
    })).collect::<Vec<_>>();

    intersections.sort_by(|a, b| a.0.distance.partial_cmp(&b.0.distance).unwrap());

    use std::f64::INFINITY;
    let mut regions = vec![
        Region {
            start: Intersection {
                point: ray.direction * -INFINITY,
                normal: -ray.direction,
                distance: -INFINITY,
            },
            end: Intersection {
                point: ray.direction * -INFINITY,
                normal: -ray.direction,
                distance: -INFINITY,
            },
            a: false,
            b: false,
        }
    ];

    for (i, (intersection, owner)) in intersections.into_iter().enumerate() {
        let (previous_a, previous_b) = {
            let previous_region = &mut regions[i];

            previous_region.end = intersection.clone().inverse();

            (previous_region.a, previous_region.b)
        };

        let (a, b) = match owner {
            Owner::A => (!previous_a, previous_b),
            Owner::B => (previous_a, !previous_b),
        };

        let region = Region {
            start: intersection.clone(),
            end: intersection,
            a,
            b,
        };

        regions.push(region);
    }

    regions.last_mut().map(|region| region.end = Intersection {
        point: ray.direction * INFINITY,
        normal: ray.direction,
        distance: INFINITY,
    });

    regions
}
