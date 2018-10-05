
type Id = usize;

use material::Material;
use shape::Shape;
use ray::Ray;
use color::Color;
use vector::Vector3;
use ray::Intersection;

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

    pub fn trace(&self, ray: Ray) -> Color {
        let mut color = Color {
            r: 0.25 * ray.direction.x.abs(),
            g: 0.25 * ray.direction.y.abs(),
            b: 0.25 * ray.direction.z.abs()
        };

        let light_direction = Vector3::new(-1.0, 2.0, -0.5).normal();

        if let Some((entry, object)) = self.get_intersection(&ray) {
            if let Some(ref material) = self.materials[object] {
                let brightness = 0.5 + 0.5 * light_direction.dot(entry.normal);
                let diffuse_color = material.color.apply_brightness(brightness);

                color = diffuse_color;
            }
        }

        color
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

    fn get_intersection(&self, ray: &Ray) -> Option<(Intersection, Id)> {
        let mut intersections = Vec::new();

        for &object in self.objects.iter() {
            if let Some(ref shape) = self.shapes[object] {
                let (entry, _) = shape.intersection(ray);

                if let Some(entry) = entry {
                    let depth = Vector3::distance(ray.origin, entry.point);

                    intersections.push((entry, depth, object));
                }
            }
        }

        let hit = intersections.into_iter()
            .min_by(|(_, depth_a, _), (_, depth_b, _)| depth_a.partial_cmp(&depth_b).unwrap());

        hit.map(|(intersection, _, object)| (intersection, object))
    }
}

