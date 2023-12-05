use std::mem::Discriminant;

use crate::math::{
    color::{self, Color},
    Vec3,
};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, dir: Vec3) -> Self {
        Self {
            origin: o,
            direction: dir,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self) -> Color {
        if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, self) {
            return color::RED;
        }
        let unit_dir = self.direction.normalize();
        let a = (unit_dir.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0, 1.0) * a
    }
}

pub fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc: Vec3 = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    return discriminant >= 0.0;
}
