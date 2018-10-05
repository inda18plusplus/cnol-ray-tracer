extern crate image;

mod vector;
mod color;

mod scene;
mod material;
mod shape;

mod ray;

use image::{
    DynamicImage,
    GenericImage
};

use vector::Vector3;
use shape::Sphere;
use scene::Scene;
use material::Material;
use color::Color;
use shape::Shape;
use ray::Ray;


fn main() {
    let scene = create_scene();

    let image = trace_scene(&scene, 800, 600);

    image.save("out.png").unwrap();
}

fn create_scene() -> Scene {
    let mut scene = Scene::new();

    let shape = Shape::Sphere(Sphere {
        center: Vector3::new(0.0, -0.5, 3.5),
        radius: 1.0
    });

    let material = Material::diffuse(Color::new(1.0, 0.0, 0.0));

    scene.add_object(shape.clone(), material);

    scene
}

fn trace_scene(scene: &Scene, image_width: u32, image_height: u32) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let ray = get_ray_from_screen(x, y, image_width, image_height);

            let color = scene.trace(ray);

            image.put_pixel(x, y, color.into());
        }
    }

    image
}


// Hjälp med den linjära algebran: https://www.scratchapixel
// .com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays
// /generating-camera-rays
fn get_ray_from_screen(x: u32, y: u32, width: u32, height: u32) -> Ray {
    let fov = 70.0f64.to_radians();
    let aspect_ratio = (width as f64) / (height as f64);

    let normal_x = 2.0 * (x as f64) / (width as f64) - 1.0;
    let normal_y = 1.0 - 2.0 * (y as f64) / (height as f64);

    let world_height = (fov / 2.0).tan();

    let direction_x = normal_x * aspect_ratio * world_height;
    let direction_y = normal_y * world_height;


    let origin = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let direction = Vector3 {
        x: direction_x,
        y: direction_y,
        z: 1.0,
    };

    Ray {
        origin,
        direction: direction.normal()
    }
}
