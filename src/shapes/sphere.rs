// src/shapes/sphere.rs

use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use glam::DVec3;
use std::sync::Arc;

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let sqrt_d = discriminant.sqrt();
            // Find the nearest root that lies in the acceptable range.
            let mut root = (-half_b - sqrt_d) / a;
            if root < t_min || root > t_max {
                root = (-half_b + sqrt_d) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }

            let t = root;
            let point = ray.point_at_parameter(t);
            let outward_normal = (point - self.center) / self.radius;
            Some(HitRecord::new(
                t,
                point,
                outward_normal,
                ray,
                self.material.clone(),
            ))
        } else {
            None
        }
    }
}