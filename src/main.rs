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

use shape::Plane;
use light::Light;
use light::PointLight;

use std::time;
use std::thread;
use std::sync::{
    Arc,
    Mutex,
    MutexGuard,
    mpsc,
    mpsc::{
        Sender,
        Receiver
    }
};


const THREAD_COUNT: usize = 4;
const BATCH_SIZE: usize = 4096;

fn main() {
    let scene = create_scene();

    let start = time::Instant::now();

    let image = trace_scene(Arc::new(scene), 800, 800);

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

fn trace_scene(scene: Arc<Scene>, image_width: u32, image_height: u32) -> DynamicImage {
    let mut pixels = Arc::new(Mutex::new(get_pixels(image_width, image_height)));

    let (sender, receiver) = mpsc::channel();

    let mut threads = Vec::new();

    for _ in 0..THREAD_COUNT {
        let pixels = pixels.clone();
        let sender = sender.clone();
        let scene = scene.clone();

        threads.push(thread::spawn(move || {
            process_pixels(pixels, sender, scene, image_width, image_height);
        }));
    }

    let image = receive_image(receiver, image_width, image_height);

    println!("Waiting for threads to join...");
    for thread in threads {
        thread.join().unwrap();
    }

    image
}


fn get_pixels(width: u32, height: u32) -> Vec<(u32, u32)> {
    let mut pixels = Vec::new();

    for y in 0..height {
        for x in 0..width {
            pixels.push((x, y));
        }
    }

    pixels
}

fn get_pixel_batch(pixels: &mut MutexGuard<Vec<(u32, u32)>>, batch_size: usize)
    -> Vec<(u32, u32)> {
    if pixels.len() < batch_size {
        pixels.split_off(0)
    } else {
        let index = pixels.len() - batch_size;
        pixels.split_off(index)
    }
}

fn process_pixels(
    pixels: Arc<Mutex<Vec<(u32, u32)>>>,
    sender: Sender<Vec<(u32, u32, Color)>>,
    scene: Arc<Scene>,
    width: u32, height: u32
) {
    loop {
        let batch = get_pixel_batch(&mut pixels.lock().unwrap(), BATCH_SIZE);
        if batch.len() == 0 {
            break;
        }

        let mut results = Vec::new();
        for (x, y) in batch {
            let ray = get_ray_from_screen(x, y, width, height);

            let color = scene.trace(ray);
            results.push((x, y, color));
        }

        sender.send(results).unwrap();
    }
}

fn receive_image(
    receiver: Receiver<Vec<(u32, u32, Color)>>,
    width: u32, height: u32
) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(width, height);

    let mut remaining_pixels = width * height;

    while let Ok(batch) = receiver.recv() {
        remaining_pixels -= batch.len() as u32;

        for (x, y, color) in batch {
            image.put_pixel(x, y, color.into());
        }

        println!("Got {} remaining", remaining_pixels);

        if remaining_pixels == 0 {
            break;
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
