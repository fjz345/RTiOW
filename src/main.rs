use std::{fs::File, io::Write, ops::Mul, process::Output, sync::Arc, thread::Thread};

use camera::Camera;
use glam::Vec3;
use material::*;
use rand::{rngs::ThreadRng, Rng};
use random::*;
use ray::SurfaceAttributes;
use renderer::render;

use crate::{
    color::*,
    material::{MATERIAL_DIELECTRIC, MATERIAL_NUM},
    math::math::*,
    progress_bar::ProgressBar,
    ray::*,
};

mod camera;
mod color;
mod interval;
mod material;
mod math;
mod progress_bar;
mod random;
mod ray;
mod renderer;
mod ringbuffer;

/* TODO:
camera direction
camera projection
*/

fn setup_world0(world: &mut HittableList) {
    const RANDOM_SURFACES_NUM: usize = 2000;
    let mut random_surfaces: Vec<SurfaceAttributes> = Vec::with_capacity(RANDOM_SURFACES_NUM);
    for _i in 0..RANDOM_SURFACES_NUM {
        let rand_vec0 = rand_vec3_range(0.0, 1.0);
        const SURFACE_EMISSIVE_CHANCE: f32 = 0.2;
        let mut rand_vec1: Vec3 = if rand_range(0.0..1.0) <= SURFACE_EMISSIVE_CHANCE {
            let mut emissive: Vec3 = rand_vec0;
            emissive * 1.0
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        };
        let rand_albedo = Color::new(rand_vec0.x, rand_vec0.y, rand_vec0.z, 0.0);
        let rand_emissve = Color::new(rand_vec1.x, rand_vec1.y, rand_vec1.z, 0.0);

        let rand_surface: SurfaceAttributes = SurfaceAttributes {
            albedo: rand_albedo,
            emissive: rand_emissve,
            ir: 1.5,
        };

        random_surfaces.push(rand_surface);
    }

    const RANDOM_SPHERES_NUM: usize = 500;
    let mut random_spheres: Vec<Sphere> = Vec::with_capacity(RANDOM_SPHERES_NUM);
    for _i in 0..RANDOM_SPHERES_NUM {
        let radius: f32 = rand_range(0.4..1.0);
        let mut rand_position: Vec3 = rand_vec3_range(-30.0, 30.0);
        rand_position.y = radius;

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

    let hittable_ground: Box<dyn Hittable + Sync + Send> = Box::new(Plane {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        material_id: 1,
        surface: SurfaceAttributes {
            albedo: Color::new(0.5, 0.5, 0.5, 1.0),
            emissive: Color::new(0.0, 0.0, 0.0, 1.0),
            ir: 1.0,
        },
    });

    world.add_hittable(hittable_ground);
    for sphere in random_spheres {
        world.add_hittable(Box::new(sphere));
    }
}

fn setup_world1(world: &mut HittableList) {
    let mut surfaces: Vec<SurfaceAttributes> = Vec::new();
    let surface: SurfaceAttributes = SurfaceAttributes {
        albedo: Color::new(1.0, 0.0, 0.0, 1.0),
        emissive: Color::new(0.0, 0.0, 0.0, 1.0),
        ir: 1.5,
    };

    let surface2: SurfaceAttributes = SurfaceAttributes {
        albedo: Color::new(0.5, 0.5, 0.5, 1.0),
        emissive: Color::new(0.0, 0.0, 0.0, 1.0),
        ir: 1.5,
    };
    surfaces.push(surface);
    surfaces.push(surface2);

    let r = 5.0;
    let mut spheres: Vec<Sphere> = Vec::new();
    let sphere: Sphere = Sphere {
        center: Vec3::new(0.0, r, -3.0),
        radius: r,
        material_id: MATERIAL_LAMBERTIAN,
        surface: surfaces[1],
    };
    let sphere2: Sphere = Sphere {
        center: Vec3::new(2.0 * r, r, -3.0),
        radius: r,
        material_id: MATERIAL_METAL,
        surface: surfaces[1],
    };
    let sphere3: Sphere = Sphere {
        center: Vec3::new(-2.0 * r, r, -3.0),
        radius: r,
        material_id: MATERIAL_DIELECTRIC,
        surface: surfaces[0],
    };
    spheres.push(sphere);
    spheres.push(sphere2);
    spheres.push(sphere3);

    let hittable_ground: Box<dyn Hittable + Sync + Send> = Box::new(Plane {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        material_id: MATERIAL_LAMBERTIAN,
        surface: SurfaceAttributes {
            albedo: Color::new(0.5, 0.5, 0.5, 1.0),
            emissive: Color::new(0.0, 0.0, 0.0, 1.0),
            ir: 1.0,
        },
    });

    world.add_hittable(hittable_ground);
    for sphere in spheres {
        world.add_hittable(Box::new(sphere));
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _rt_guard = rt.enter();

    let mut camera: Camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 500;
    camera.fov = 40.0;
    camera.samples_per_pixel = 10;
    camera.max_ray_per_pixel = 10;
    camera.position = Vec3::new(-30.0, 6.0, -20.0);
    let look_at_position = Vec3::new(0.0, 0.0, 0.0);
    camera.look_at(look_at_position, Vec3::new(0.0, 1.0, 0.0));

    camera.defocus_angle = 0.6 * 0.5;
    camera.focus_dist = (camera.position - look_at_position).length();

    let mut world0: HittableList = HittableList::new();
    setup_world0(&mut world0);
    let mut world1: HittableList = HittableList::new();
    setup_world1(&mut world1);
    world0.merge(world1);

    let mut world = world0;
    let render_file_path = "../img/render_test.ppm";
    render(&mut world, &mut camera, render_file_path).unwrap();
}
