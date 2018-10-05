mod vector;
mod color;

mod scene;
mod material;
mod shape;

use vector::Vector3;
use shape::Sphere;
use scene::Scene;
use material::Material;
use color::Color;
use shape::Shape;

fn main() {
    println!("Hello, world!");

    let mut scene = Scene::new();

    let shape = Shape::Sphere(Sphere {
        center: Vector3::new(0.0, 0.0, 0.0),
        radius: 1.0
    });

    let material = Material::diffuse(Color::new(1.0, 0.0, 0.0));

    scene.add_object(shape.clone(), material);

    println!("{:?}", shape);
}
