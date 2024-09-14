// src/materials.rs

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::{random_in_unit_sphere, random_unit_vector};
use glam::DVec3;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Ray, DVec3)>;
}

// Lambertian Material
pub struct Lambertian {
    pub albedo: DVec3,
}

impl Lambertian {
    pub fn new(albedo: DVec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Ray, DVec3)> {
        let scatter_direction = hit_record.normal + random_unit_vector();
        let scattered = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

// Metal Material
pub struct Metal {
    pub albedo: DVec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: DVec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Ray, DVec3)> {
        let reflected = reflect(ray_in.direction.normalize(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        let attenuation = self.albedo;

        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

// Dielectric Material
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Ray, DVec3)> {
        let attenuation = DVec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if hit_record.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let reflect_prob = schlick(cos_theta, etai_over_etat);

        // Use rand::random::<f64>() to generate a random number
        let direction = if etai_over_etat * sin_theta > 1.0 || rand::random::<f64>() < reflect_prob {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, etai_over_etat)
        };

        let scattered = Ray::new(hit_record.point, direction);
        Some((scattered, attenuation))
    }
}

// Helper Functions
fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(uv: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}