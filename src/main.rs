// src/main.rs

mod camera;
mod hittable;
mod materials;
mod ray;
mod renderer;
mod shapes;
mod utils;

use camera::Camera;
use hittable::Hittable;
use materials::{Dielectric, Lambertian, Metal};
use renderer::Renderer;
use shapes::{Cube, Sphere};
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;
use std::time::Instant;

// Use DVec3 from glam
use glam::DVec3;

fn main() {
    // Image settings
    let aspect_ratio = 16.0 / 9.0;
    let width = 800;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 512;

    // World setup
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    // Materials
    let ground_material = Arc::new(Lambertian::new(DVec3::new(0.8, 0.8, 0.0)));
    let center_material = Arc::new(Lambertian::new(DVec3::new(0.1, 0.2, 0.5)));
    let left_material = Arc::new(Dielectric::new(1.5));
    let right_material = Arc::new(Metal::new(DVec3::new(0.8, 0.6, 0.2), 0.0));
    let cube_material = Arc::new(Lambertian::new(DVec3::new(0.7, 0.3, 0.3)));

    // Spheres
    world.push(Box::new(Sphere::new(
        DVec3::new(0.0, -100.5, -1.0),
        100.0,
        ground_material,
    )));
    world.push(Box::new(Sphere::new(
        DVec3::new(0.0, 0.0, -1.0),
        0.5,
        center_material,
    )));
    world.push(Box::new(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.0),
        0.5,
        left_material.clone(),
    )));
    world.push(Box::new(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.0),
        -0.45, // Negative radius for inner sphere to create a hollow glass sphere
        left_material,
    )));
    world.push(Box::new(Sphere::new(
        DVec3::new(1.0, 0.0, -1.0),
        0.5,
        right_material,
    )));

    // Cube
    world.push(Box::new(Cube::new(
        DVec3::new(-1.5, -0.5, -2.0),
        DVec3::new(-1.0, 0.5, -1.5),
        cube_material,
    )));

    // Camera setup
    let lookfrom = DVec3::new(3.0, 3.0, 2.0);
    let lookat = DVec3::new(0.0, 0.0, -1.0);
    let vup = DVec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let focus_dist = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    // Renderer setup
    let renderer = Renderer::new(width, height, samples_per_pixel);

    // Output file
    let file = File::create("output.ppm").expect("Unable to create output file");
    let mut output = BufWriter::new(file);

    // Start the timer
    let start_time = Instant::now();

    let world = Arc::new(world);   // Wrap world in Arc
    let camera = Arc::new(camera); // Wrap camera in Arc
    
    renderer.render(world, camera, &mut output);

    let duration = start_time.elapsed();
    println!("Rendering completed in: {:?}", duration);
}