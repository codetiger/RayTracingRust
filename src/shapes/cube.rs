// src/shapes/cube.rs

use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use glam::DVec3;
use std::sync::Arc;

pub struct Cube {
    pub min: DVec3,
    pub max: DVec3,
    pub material: Arc<dyn Material>,
}

impl Cube {
    pub fn new(min: DVec3, max: DVec3, material: Arc<dyn Material>) -> Self {
        Cube { min, max, material }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min_initial: f64, t_max_initial: f64) -> Option<HitRecord> {
        let inv_dir = DVec3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        let mut t_min = t_min_initial;
        let mut t_max = t_max_initial;

        for i in 0..3 {
            let origin_component = ray.origin[i];
            let inv_direction_component = inv_dir[i];
            let min_component = self.min[i];
            let max_component = self.max[i];

            let t0 = ((min_component - origin_component) * inv_direction_component)
                .min((max_component - origin_component) * inv_direction_component);
            let t1 = ((min_component - origin_component) * inv_direction_component)
                .max((max_component - origin_component) * inv_direction_component);

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min {
                return None;
            }
        }

        let t = t_min;
        let point = ray.point_at_parameter(t);

        // Calculate outward normal based on which face was hit
        let epsilon = 1e-6;
        let mut outward_normal = DVec3::ZERO;
        if (point.x - self.min.x).abs() < epsilon {
            outward_normal.x = -1.0;
        } else if (point.x - self.max.x).abs() < epsilon {
            outward_normal.x = 1.0;
        } else if (point.y - self.min.y).abs() < epsilon {
            outward_normal.y = -1.0;
        } else if (point.y - self.max.y).abs() < epsilon {
            outward_normal.y = 1.0;
        } else if (point.z - self.min.z).abs() < epsilon {
            outward_normal.z = -1.0;
        } else if (point.z - self.max.z).abs() < epsilon {
            outward_normal.z = 1.0;
        }

        Some(HitRecord::new(
            t,
            point,
            outward_normal,
            ray,
            self.material.clone(),
        ))
    }
}