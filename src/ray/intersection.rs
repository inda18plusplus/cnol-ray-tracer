use vector::Vector3;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub point: Vector3,
    pub normal: Vector3,
    pub distance: f64
}

pub type EntryExit = (Intersection, Intersection);

impl Intersection {
    /// Inverts the normal
    pub fn inverse(self) -> Intersection {
        Intersection {
            normal: -self.normal,
            ..self
        }
    }
}
