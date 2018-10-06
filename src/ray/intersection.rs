use vector::Vector3;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub point: Vector3,
    pub normal: Vector3
}

pub type EntryExit = (Option<Intersection>, Option<Intersection>);
