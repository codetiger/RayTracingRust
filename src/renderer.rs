use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::utils::clamp;
use glam::DVec3;
use rayon::prelude::*;
use std::io::Write;
use std::sync::Arc;
use rand::Rng;

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize, samples_per_pixel: usize) -> Self {
        Renderer {
            width,
            height,
            samples_per_pixel,
        }
    }

    pub fn render<W: Write + Send>(
        &self,
        world: Arc<dyn Hittable + Send + Sync>,
        camera: Arc<Camera>,
        output: &mut W,
    ) {
        let mut pixels = vec![DVec3::ZERO; self.width * self.height];

        pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let i = index % self.width;
                let j = self.height - 1 - index / self.width;

                let mut color = DVec3::ZERO;
                let mut rng = rand::thread_rng();

                for _s in 0..self.samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / self.width as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / self.height as f64;

                    let ray = camera.get_ray(u, v);
                    color += self.ray_color(&ray, &*world, 0);
                }

                color /= self.samples_per_pixel as f64;
                color = DVec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());

                *pixel = color;
            });

        writeln!(output, "P3\n{} {}\n255", self.width, self.height).unwrap();

        for color in pixels.iter() {
            let ir = (256.0 * clamp(color.x, 0.0, 0.999)) as i32;
            let ig = (256.0 * clamp(color.y, 0.0, 0.999)) as i32;
            let ib = (256.0 * clamp(color.z, 0.0, 0.999)) as i32;

            writeln!(output, "{} {} {}", ir, ig, ib).unwrap();
        }
    }

    fn ray_color(
        &self,
        ray: &Ray,
        world: &dyn Hittable,
        depth: u32,
    ) -> DVec3 {
        if depth >= 50 {
            return DVec3::ZERO;
        }

        if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
            if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec) {
                return attenuation * self.ray_color(&scattered, world, depth + 1);
            } else {
                return DVec3::ZERO;
            }
        } else {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * DVec3::new(1.0, 1.0, 1.0) + t * DVec3::new(0.5, 0.7, 1.0)
        }
    }
}