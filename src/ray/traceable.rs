
pub trait Traceable{
    fn intersections(&self, ray: Ray) -> Vec<Intersection>;
}

