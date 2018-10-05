
type Id = usize;

use material::Material;
use shape::Shape;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Id>,
    materials: Vec<Option<Material>>,
    shapes: Vec<Option<Shape>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            materials: Vec::new(),
            shapes: Vec::new(),
        }
    }

    pub fn add_object(&mut self, shape: Shape, material: Material) -> Id {
        let id = self.generate_next_id();

        self.materials[id] = Some(material);
        self.shapes[id] = Some(shape);

        id
    }
}


impl Scene {
    fn generate_next_id(&mut self) -> Id {
        let id = self.objects.len();
        self.resize_to_fit(id);

        id
    }

    fn resize_to_fit(&mut self, id: Id) {
        self.objects.push(id);
        self.materials.push(None);
        self.shapes.push(None);
    }
}

