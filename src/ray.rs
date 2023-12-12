use std::{default, mem::Discriminant, ops::DerefMut};

use glam::Vec3;

use crate::color::Color;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, dir: Vec3) -> Self {
        Self {
            origin: o,
            direction: dir.normalize(),
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self, world: &HittableList) -> Color {
        let mut hit_result: HitResult = HitResult::default();
        if world.hit(self, 0.0, 1000.0, &mut hit_result) {
            let col = (hit_result.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
            return Color::new(col.x, col.y, col.z, 1.0);
        }

        // BG
        let unit_dir = self.direction.normalize();
        let a = (unit_dir.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0, 1.0) * a
    }
}

#[derive(Clone, Copy, Default)]
pub struct HitResult {
    location: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitResult {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_result: &mut HitResult) -> bool;
    fn clone_dyn(&self) -> Box<dyn Hittable>;
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_result: &mut HitResult) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a: f32 = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let discriminant_sqrt = discriminant.sqrt();

        let t0: f32 = (-half_b - discriminant_sqrt) / a;
        let t1: f32 = (-half_b + discriminant_sqrt) / a;
        let mut t = t1;

        if t <= t_min || t_max <= t {
            t = t0;
            if t <= t_min || t_max <= t {
                return false;
            }
        }
        t = t0.min(t1);

        hit_result.t = t;
        hit_result.location = ray.at(hit_result.t);
        hit_result.normal = (hit_result.location - self.center) / self.radius;
        hit_result.set_face_normal(ray, hit_result.normal);

        return true;
    }
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add_hittable(&mut self, hittable: &Box<dyn Hittable>) {
        self.list.push(hittable.clone());
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_result: &mut HitResult) -> bool {
        let mut temp_hit_result: HitResult = HitResult::default();
        let mut hit = false;
        let mut closest_so_far = t_max;

        for object in self.list.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_hit_result) {
                hit = true;
                closest_so_far = temp_hit_result.t;
                *hit_result = temp_hit_result;
            }
        }

        return hit;
    }
}
