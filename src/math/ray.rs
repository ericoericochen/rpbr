use crate::math::vec::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
    // medium: Option<&>
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.,
        }
    }

    pub fn time(&self, time: f64) -> Self {
        Self { time, ..*self }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

pub struct RayDifferential {
    ray: Ray,
    has_differentials: bool,
    rx_origin: Point3,
    ry_origin: Point3,
    rx_direction: Vec3,
    ry_direction: Vec3,
}

impl RayDifferential {
    pub fn scale_differentials(&mut self, s: f64) {
        self.rx_origin = self.ray.origin + (self.rx_origin - self.ray.origin) * s;
        self.ry_origin = self.ray.origin + (self.ry_origin - self.ray.origin) * s;
        self.rx_direction = self.ray.direction + (self.rx_direction - self.ray.direction) * s;
        self.ry_direction = self.ray.direction + (self.ry_direction - self.ray.direction) * s;
    }
}

// pub ray_differiential_from_ray(ray: Ray) {

// }

mod tests {
    use super::*;
}
