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
const BATCH_SIZE: usize = 512;

fn main() {
    let scene = create_scene();

    let start = time::Instant::now();

    let res = 800;
    let image = trace_scene(Arc::new(scene), res, res);

    let end = time::Instant::now();
    let duration = end - start;
    let seconds = duration.as_secs() as f64 + 1.0e-9 * duration.subsec_nanos() as f64;
    println!("Done in {:.3} seconds ({:.1} fps)", seconds, 1.0 / seconds);

    image.save("out.png").unwrap();
}

fn create_scene() -> Scene {
    let mut scene = Scene::new();

    // Red sphere
    let shape = sphere([-1.0, -2.0, 5.5], 0.75);
    let material = Material::new(Color::new(1.0, 0.0, 0.0), 0.0, 0.3);
    scene.add_object(shape, material);

    // Cyan sphere
    let shape = sphere([1.2, 1.25, 5.0], 0.75);
    let material = Material::new(Color::new(0.0, 1.0, 1.0), 0.0, 0.1);
    scene.add_object(shape, material);

    // Difference of two spheres
    let a = Box::new(sphere([1.2, -2.0, 4.5], 1.0));
    let b = Box::new(sphere([0.7, -1.1, 4.5], 1.0));
    let c = Box::new(sphere([0.5, -2.0, 4.3], 1.0));
    let difference = Box::new(Shape::Difference(a, b));
    let shape = Shape::Intersection(difference, c);

    let material = Material::new(Color::new(1.0, 1.0, 1.0), 1.0, 0.1);
    scene.add_object(shape, material);


    // Floor
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(0.0, -3.0, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
    });

    let material = Material::new(Color::new(0.0, 0.1, 0.1), 0.0, 1.0);
    scene.add_object(shape, material);

    // Right wall
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(3.0, 0.0, 0.0),
        normal: Vector3::new(-1.0, 0.0, 0.0),
    });
    let material = Material::new(Color::new(0.0, 1.0, 0.0), 0.1, 0.3);
    scene.add_object(shape, material);

    // Left wall
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(-3.0, 0.0, 0.0),
        normal: Vector3::new(1.0, 0.0, 0.0),
    });
    let material = Material::new(Color::new(1.0, 1.0, 0.0), 0.1, 0.3);
    scene.add_object(shape, material);

    // Back wall
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(0.0, 0.0, 7.0),
        normal: Vector3::new(0.0, 0.0, -1.0),
    });
    let material = Material::new(Color::new(0.0, 0.0, 0.0), 0.0, 1.0);
    scene.add_object(shape, material);

    // Front wall
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(0.0, 0.0, -1.0),
        normal: Vector3::new(0.0, 0.0, 1.0),
    });
    let material = Material::new(Color::new(0.0, 0.0, 0.0), 0.0, 1.0);
    scene.add_object(shape, material);

    // Ceiling
    let shape = Shape::Plane(Plane {
        origin: Vector3::new(0.0, 3.0, 0.0),
        normal: Vector3::new(0.0, -1.0, 0.0),
    });
    let material = Material::new(Color::new(0.0, 0.0, 1.0), 0.1, 0.3);
    scene.add_object(shape, material);

    // Light
    let light = Light::Point(PointLight {
        point: Vector3::new(-2.2, 2.2, 2.0),
        color: Color::white(),
        size: 0.2
    });
    scene.add_light(light);

    let light = Light::Point(PointLight {
        point: Vector3::new(-1.0, 0.2, 1.0),
        color: Color::white(),
        size: 0.4
    });
    scene.add_light(light);


    scene
}


fn sphere(center: [f64; 3], radius: f64) -> Shape {
    Shape::Sphere(Sphere {
        center: Vector3 {
            x: center[0],
            y: center[1],
            z: center[2],
        },
        radius
    })
}


fn trace_scene(scene: Arc<Scene>, image_width: u32, image_height: u32) -> DynamicImage {
    // Render at twice the scale and downsample
    let (render_width, render_height) = (image_width * 2, image_height * 2);

    let pixels = Arc::new(Mutex::new(get_pixels(render_width, render_height)));

    let (sender, receiver) = mpsc::channel();

    let mut threads = Vec::new();

    for _ in 0..THREAD_COUNT {
        let pixels = pixels.clone();
        let sender = sender.clone();
        let scene = scene.clone();

        threads.push(thread::spawn(move || {
            process_pixels(pixels, sender, scene, render_width, render_height);
        }));
    }

    let image = receive_image(receiver, render_width, render_height);

    println!("Waiting for threads to join...");
    for thread in threads {
        thread.join().unwrap();
    }

    image.resize(image_width, image_height, image::FilterType::Triangle)
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

    let start_time = time::Instant::now();

    while let Ok(batch) = receiver.recv() {
        remaining_pixels -= batch.len() as u32;

        for (x, y, color) in batch {
            image.put_pixel(x, y, color.into());
        }

        let current_time = time::Instant::now();

        let percentage = 1.0 - (remaining_pixels as f64 / (width * height) as f64);

        let duration = current_time - start_time;
        let time_elapsed = duration.as_secs() as f64 / 60.0;
        let time_remaining = time_elapsed / percentage - time_elapsed;

        println!("{} pixels remaining ({:.2}% in {:.1} minutes, approx. {:.1} minutes left)",
                 remaining_pixels,
                 percentage * 100.0,
                 time_elapsed,
                 time_remaining);

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
        z: -1.0,
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
