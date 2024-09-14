// src/camera.rs

use crate::ray::Ray;
use crate::utils::random_in_unit_disk;
use glam::DVec3;

pub struct Camera {
    origin: DVec3,
    lower_left_corner: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    u: DVec3,
    v: DVec3,
    w: DVec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: DVec3,
        lookat: DVec3,
        vup: DVec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
    
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical
            - self.origin - offset;
    
        Ray::new(self.origin + offset, direction)
    }
}