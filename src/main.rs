extern crate image;
extern crate rand;

mod vector;
mod color;
mod ray;

mod scene;
mod material;
mod shape;
mod light;

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

use std::time;
use shape::Plane;
use light::Light;
use light::PointLight;

fn main() {
    let scene = create_scene();

    let start = time::Instant::now();

    let image = trace_scene(&scene, 800, 800);

    let end = time::Instant::now();
    let duration = end - start;
    let seconds = duration.as_secs() as f64 + 1.0e-9 * duration.subsec_nanos() as f64;
    println!("Done in {:.3} seconds ({:.1} fps)", seconds, 1.0 / seconds);

    image.save("out.png").unwrap();
}

fn create_scene() -> Scene {
    let mut scene = Scene::new();

    let shape = Shape::Sphere(Sphere {
        center: Vector3::new(0.0, -1.0, 5.0),
        radius: 1.0
    });

    let material = Material::diffuse(Color::new(1.0, 0.0, 0.0));
    scene.add_object(shape, material);

    // Floor
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(0.0, -2.0, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
    });

    let material = Material::diffuse(Color::new(1.0, 1.0, 1.0));
    scene.add_object(shape, material);

    // Right wall
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(3.0, 0.0, 0.0),
        normal: Vector3::new(-1.0, 0.0, 0.0),
    });
    let material = Material::diffuse(Color::new(0.0, 1.0, 0.0));
    scene.add_object(shape, material);

    // Left wall
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(-3.0, 0.0, 0.0),
        normal: Vector3::new(1.0, 0.0, 0.0),
    });
    let material = Material::diffuse(Color::new(1.0, 1.0, 0.0));
    scene.add_object(shape, material);

    // Back wall
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(0.0, 0.0, 7.0),
        normal: Vector3::new(0.0, 0.0, -1.0),
    });
    let material = Material::diffuse(Color::new(1.0, 0.0, 1.0));
    scene.add_object(shape, material);

    // Front wall
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(0.0, 0.0, -1.0),
        normal: Vector3::new(0.0, 0.0, 1.0),
    });
    let material = Material::diffuse(Color::new(1.0, 1.0, 1.0));
    scene.add_object(shape, material);

    // Ceiling
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(0.0, 3.0, 0.0),
        normal: Vector3::new(0.0, -1.0, 0.0),
    });
    let material = Material::diffuse(Color::new(0.0, 0.0, 1.0));
    scene.add_object(shape, material);

    // Light
    let light = Light::Point(PointLight {
        point: Vector3::new(-0.2, 2.2, 5.0),
        color: Color::white(),
        size: 0.05
    });
    scene.add_light(light);

    let light = Light::Point(PointLight {
        point: Vector3::new(2.0, 2.2, 5.0),
        color: Color::white(),
        size: 0.25
    });
    scene.add_light(light);


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

        println!("{:.3}%", (y as f64 + 1.0) / (image_height as f64) * 100.0);
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
