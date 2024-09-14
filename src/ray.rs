// src/ray.rs

use glam::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }
}