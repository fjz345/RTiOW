use std::{fs::File, io::Write, process::Output};

use camera::Camera;
use glam::Vec3;

use crate::{
    color::{color::color_to_u8, Color},
    progress_bar::ProgressBar,
    ray::{Hittable, HittableList, Ray, Sphere},
};

mod camera;
mod color;
mod interval;
mod math;
mod progress_bar;
mod ray;

fn main() {
    let mut camera: Camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 4;

    let mut world: HittableList = HittableList::new();
    let hittable_circle: Box<dyn Hittable> = Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    });
    let hittable_ground: Box<dyn Hittable> = Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    });
    world.add_hittable(hittable_circle);
    world.add_hittable(hittable_ground);

    camera.render(&world);
}
