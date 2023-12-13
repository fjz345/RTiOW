use std::{fs::File, io::Write, process::Output, thread::Thread};

use camera::Camera;
use glam::Vec3;
use rand::{rngs::ThreadRng, Rng};
use random::*;
use ray::SurfaceAttributes;

use crate::{color::*, material::MATERIAL_NUM, math::math::*, progress_bar::ProgressBar, ray::*};

mod camera;
mod color;
mod interval;
mod material;
mod math;
mod progress_bar;
mod random;
mod ray;

fn setup_world(world: &mut HittableList) {
    let mut rng: ThreadRng = rand::thread_rng();
    const random_surfaces_num: usize = 2000;
    let mut random_surfaces: Vec<SurfaceAttributes> = Vec::with_capacity(random_surfaces_num);
    for i in 0..random_surfaces_num {
        let rand_vec0 = rand_vec3_range(0.0, 1.0);
        let rand_vec1 = rand_vec3_range(0.0, 1.0);
        let rand_albedo = Color::new(rand_vec0.x, rand_vec0.y, rand_vec0.z, 0.0);
        let rand_emissve = Color::new(rand_vec1.x, rand_vec1.y, rand_vec1.z, 0.0);

        let rand_surface: SurfaceAttributes = SurfaceAttributes {
            albedo: rand_albedo,
            emissive: rand_emissve,
        };

        random_surfaces.push(rand_surface);
    }

    const random_spheres_num: usize = 200;
    let mut random_spheres: Vec<Sphere> = Vec::with_capacity(random_spheres_num);
    for i in 0..random_spheres_num {
        let mut rand_position: Vec3 = rand_vec3_range(-30.0, 30.0);
        rand_position.y = 0.0;
        let radius: f32 = rand_range(0.1..2.2);

        let rand_surface_index: usize = rand_range(0..random_surfaces.len());
        let rand_material_id: i32 = rand_range(0..MATERIAL_NUM);

        let rand_sphere: Sphere = Sphere {
            center: rand_position,
            radius: radius,
            material_id: rand_material_id,
            surface: random_surfaces[rand_surface_index],
        };

        random_spheres.push(rand_sphere);
    }

    let hittable_ground: Box<dyn Hittable> = Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material_id: 1,
        surface: SurfaceAttributes {
            albedo: Color::new(0.5, 0.5, 0.5, 1.0),
            emissive: Color::new(0.0, 0.0, 0.0, 1.0),
        },
    });
    world.add_hittable(hittable_ground);
    for sphere in random_spheres {
        world.add_hittable(Box::new(sphere));
    }
}

fn main() {
    let mut camera: Camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 20;
    camera.max_ray_per_pixel = 3;

    let mut world: HittableList = HittableList::new();
    setup_world(&mut world);

    camera.render(&world);
}
