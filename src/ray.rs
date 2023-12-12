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
pub struct HitResult {
    pub location: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material_id: i32,
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
    fn hit(&self, ray: &Ray, interval: Interval, hit_result: &mut HitResult) -> bool;
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
}

impl Hittable for Sphere {
    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn hit(&self, ray: &Ray, interval: Interval, hit_result: &mut HitResult) -> bool {
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

        if !interval.surrounds(t) {
            t = t0;
            if !interval.surrounds(t) {
                return false;
            }
        }
        t = t0.min(t1);

        hit_result.material_id = self.material_id;
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

    pub fn add_hittable(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn hit(&self, ray: &Ray, interval: Interval, hit_result: &mut HitResult) -> bool {
        let mut temp_hit_result: HitResult = HitResult::default();
        let mut hit = false;
        let mut closest_so_far = interval.max;

        for object in self.list.iter() {
            if object.hit(
                ray,
                Interval {
                    min: interval.min,
                    max: closest_so_far,
                },
                &mut temp_hit_result,
            ) {
                hit = true;
                closest_so_far = temp_hit_result.t;
                *hit_result = temp_hit_result;
            }
        }

        return hit;
    }
}
