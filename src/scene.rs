
type Id = usize;

use material::Material;
use shape::Shape;
use ray::Ray;
use color::Color;
use vector::Vector3;
use ray::Intersection;
use light::Light;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Id>,
    materials: Vec<Option<Material>>,
    shapes: Vec<Option<Shape>>,

    lights: Vec<Light>
}

const MAX_BOUNCES: usize = 40;
const MAX_LIGHT_SAMPLES: usize = 20;
const MAX_BOUNCE_SAMPLES: usize = 5;

struct RayProperties {
    bounces: usize,
    light_samples: usize,
    bounce_samples: usize
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            materials: Vec::new(),
            shapes: Vec::new(),

            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, shape: Shape, material: Material) -> Id {
        let id = self.generate_next_id();

        self.materials[id] = Some(material);
        self.shapes[id] = Some(shape);

        id
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn trace(&self, ray: Ray) -> Color {
        let properties = RayProperties {
            bounces: MAX_BOUNCES,
            light_samples: MAX_LIGHT_SAMPLES,
            bounce_samples: MAX_BOUNCE_SAMPLES,
        };

        if let Some(color) = self.trace_ray_color(&ray, properties) {
            color
        } else {
            Color {
                r: 0.25 * ray.direction.x.abs(),
                g: 0.25 * ray.direction.y.abs(),
                b: 0.25 * ray.direction.z.abs()
            }
        }
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


    fn trace_ray_color(&self, ray: &Ray, properties: RayProperties) -> Option<Color> {
        if properties.bounces == 0 {
            return None;
        }

        if let Some((entry, object)) = self.get_intersection(&ray) {
            if let Some(ref material) = self.materials[object] {
                let ambient_color = material.color.apply_brightness(0.1);

                let point = entry.point - ray.direction * 0.0001;
                let adjusted_entry = Intersection {point, normal: entry.normal};

                let light_color = self.light_color(adjusted_entry.clone(), properties.light_samples)
                    .multiply(material.color);

                let bounce_color = self.bounce_color(ray, adjusted_entry, properties, material);

                return Some(ambient_color.add(light_color).add(bounce_color));
            }
        }

        None
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

    fn light_color(&self, entry: Intersection, samples: usize) -> Color {
        let mut color = Color::black();

        for light in self.lights.iter() {
            for _ in 0..samples {
                if let Some(distance) = self.distance_to_light(entry.point, light) {
                    let diffuse = Vector3::dot(entry.point - light.sample_point(), -entry.normal);

                    let brightness = light.brightness(distance) * if diffuse > 0.0 {diffuse} else {0.0};

                    let light_color = light.color().apply_brightness(brightness / samples as f64);

                    color = color.add(light_color);
                }
            }
        }

        color
    }

    fn bounce_color(
        &self,
        ray: &Ray,
        entry: Intersection,
        properties: RayProperties,
        material: &Material
    ) -> Color {
        let mut bounce_color = Color::black();

        for _ in 0..properties.bounce_samples {
            let bounce_ray = Ray::scatter(ray, entry.clone(), material.roughness);

            let bounce_properties = RayProperties {
                bounces: properties.bounces - 1,
                light_samples: (properties.light_samples as f64 / 4.0).ceil() as usize,
                bounce_samples: (properties.bounce_samples as f64 / 5.0).ceil() as usize,
            };

            if let Some(color) = self.trace_ray_color(&bounce_ray, bounce_properties) {
                bounce_color = bounce_color.add(
                    color.apply_brightness(material.reflectiveness / properties.bounce_samples as f64)
                );
            }
        }

        bounce_color
    }

    fn distance_to_light(&self, point: Vector3, light: &Light) -> Option<f64> {
        let delta = light.sample_point() - point;

        let ray = Ray {
            origin: point,
            direction: delta.normal(),
        };

        if let Some((entry, _)) = self.get_intersection(&ray) {
            let light_depth = delta.length();
            let entry_depth = Vector3::distance(entry.point, point);

            if light_depth < entry_depth {
                return Some(light_depth);
            }
        }

        None
    }
}

