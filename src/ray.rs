use std::{default, f32::INFINITY, mem::Discriminant, ops::DerefMut};

use crate::interval::Interval;
use glam::Vec3;

use crate::color::Color;

#[derive(Default)]
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
}

#[derive(Clone, Copy, Default)]
pub struct SurfaceAttributes {
    pub albedo: Color,
    pub emissive: Color,
}

#[derive(Clone, Copy, Default)]
pub struct HitResult {
    pub location: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material_id: i32,
    pub surface: SurfaceAttributes,
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
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitResult>;
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
    pub material_id: i32,
    pub surface: SurfaceAttributes,
}

impl Hittable for Sphere {
    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitResult> {
        let oc: Vec3 = ray.origin - self.center;
        let a: f32 = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();

        let t0: f32 = (-half_b - discriminant_sqrt) / a;
        let t1: f32 = (-half_b + discriminant_sqrt) / a;
        let mut t = t1;

        if !interval.surrounds(t) {
            t = t0;
            if !interval.surrounds(t) {
                return None;
            }
        }
        t = t0.min(t1);

        let mut hit_result: HitResult = HitResult {
            location: ray.at(t),
            normal: (ray.at(t) - self.center) / self.radius,
            t: t,
            front_face: false,
            material_id: self.material_id,
            surface: self.surface,
        };
        hit_result.set_face_normal(ray, hit_result.normal);

        return Some(hit_result);
    }
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add_hittable(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn hit_all(&self, ray: &Ray, interval: Interval) -> Option<HitResult> {
        let mut hit = false;
        let mut closest_so_far = interval.max;
        let mut hit_result = HitResult::default();
        for object in self.list.iter() {
            if let Some(temp_hit_result) = object.hit(
                ray,
                Interval {
                    min: interval.min,
                    max: closest_so_far,
                },
            ) {
                hit = true;
                closest_so_far = temp_hit_result.t;
                hit_result = temp_hit_result;
            }
        }

        if !hit {
            return None;
        }

        return Some(hit_result);
    }
}
