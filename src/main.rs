use std::{fs::File, io::Write, process::Output};

use glam::Vec3;

use crate::{
    color::{color::color_to_u8, Color},
    progress_bar::ProgressBar,
    ray::{Hittable, HittableList, Ray, Sphere},
};

mod camera;
mod color;
mod math;
mod progress_bar;
mod ray;

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i64 = 400;
    let image_height: i64 = ((image_width as f64 / aspect_ratio) as i64).max(1);
    let image_size = (image_width, image_height);

    let mut progress_bar: ProgressBar = ProgressBar::new((image_width * image_height) as f64, 20);

    let mut image_ppm: String = String::new();
    image_ppm += &format!("P3\n{} {}\n255\n", image_width, image_height).to_string();

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

    let viewport_aspectratio = (image_width as f64) / (image_height as f64);
    let viewport_height = 2.0;
    let viewport_width = viewport_height * viewport_aspectratio;
    let viewport_size: (f64, f64) = (viewport_width, viewport_height);

    let mut camera_center = Vec3::new(0.0, 0.0, 0.0);
    let mut focal_length = 1.0;

    let viewport_u = Vec3::new(viewport_width as f32, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height as f32, 0.0);

    let pixel_delta_u: Vec3 = viewport_u / image_width as f32;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: Vec3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel_center: Vec3 =
                pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
            let pixel_dir = (pixel_center - camera_center);

            let ray: Ray = Ray::new(pixel_center, pixel_dir);

            let texel_color: Color = ray.ray_color(&world);
            let texel_color_u8 = color_to_u8(&texel_color);

            let ir = texel_color_u8[0];
            let ig = texel_color_u8[1];
            let ib = texel_color_u8[2];

            image_ppm += &ir.to_string();
            image_ppm += &' '.to_string();
            image_ppm += &ig.to_string();
            image_ppm += &' '.to_string();
            image_ppm += &ib.to_string();
            image_ppm += &'\n'.to_string();

            if (x + y * image_width) % progress_bar.calc_increment() as i64 == 0 {
                progress_bar.print_progress_percent();
                progress_bar.inc();
            }
        }
    }
    assert!(progress_bar.is_finished());
    println!("Render finished!");

    let render_file_path = "img/render.ppm";
    println!("Saving to file {}...", render_file_path);
    let mut render_file = File::create(render_file_path).unwrap();
    render_file.write_all(image_ppm.as_bytes()).unwrap();
}
