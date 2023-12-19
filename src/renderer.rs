use std::{
    cell::RefCell,
    fs::File,
    io::Write,
    sync::Arc,
    thread::{self, JoinHandle, ScopedJoinHandle},
    time::SystemTime,
};

use palette::{Clamp, Srgba};

use crate::{
    camera::Camera,
    color::{color::color_to_u8_srgba, Color},
    interval::Interval,
    material::scatter,
    progress_bar::ProgressBar,
    ray::{HittableList, Ray},
};

pub fn render(world: &mut HittableList, camera: &mut Camera) {
    camera.initialize();

    let (image_width, image_height) = camera.get_image_xy();

    let time_start = SystemTime::now();

    let mut image_ppm: String = String::new();
    image_ppm += &format!("P3\n{} {}\n255\n", image_width, image_height).to_string();

    render_inner(world, camera, &mut image_ppm);
    println!("Render finished!");

    let time_now = SystemTime::now();
    let since_the_epoch = time_now
        .duration_since(time_start)
        .expect("Time went backwards");
    println!("Render took: {:?} seconds", since_the_epoch.as_secs_f32());

    let render_file_path = "img/render.ppm";
    println!("Saving to file {}...", render_file_path);
    let mut render_file = File::create(render_file_path).unwrap();
    render_file.write_all(image_ppm.as_bytes()).unwrap();
}

fn render_inner_thread(world: &HittableList, camera: &Camera, x: i32, y: i32) -> Color {
    let mut sum_texel_color: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    for _aa in 0..camera.samples_per_pixel {
        let ray: Ray = camera.get_ray(x, y);

        let texel_color: Color = ray_color(&ray, camera.max_ray_per_pixel, &world);
        sum_texel_color += texel_color;
    }

    sum_texel_color
}

pub fn render_inner(world: &HittableList, camera: &Camera, image_string: &mut String) {
    let (image_width, image_height) = camera.get_image_xy();
    let total_ray_pixel_tasks: i32 = image_width * image_height;
    let mut progress_bar: ProgressBar =
        ProgressBar::new((image_width * image_height) as f64, 20 as usize);

    const MULTITHREAD_ENABLE: bool = true;
    if MULTITHREAD_ENABLE {
        thread::scope(|s| {
            let num_threads_to_spawn: i32 = total_ray_pixel_tasks;
            let mut all_thread_handles: Vec<ScopedJoinHandle<Color>> =
                Vec::with_capacity(num_threads_to_spawn as usize);

            for i in 0..num_threads_to_spawn {
                let thread_id = i;
                let thread_x: i32 = thread_id % image_width;
                let thread_y = thread_id / image_width;
                let thread_handle = s.spawn(move || {
                    let texel_color = render_inner_thread(world, camera, thread_x, thread_y);
                    return texel_color;
                });
                all_thread_handles.push(thread_handle);
            }

            let mut thread_results: Vec<Color> = Vec::new();
            for thread_handle in all_thread_handles {
                let thread_result = thread_handle.join().unwrap();
                thread_results.push(thread_result);
            }

            for (i, result) in thread_results.iter().enumerate() {
                write_color(image_string, *result, camera.samples_per_pixel);

                if i as i32 % progress_bar.calc_increment() as i32 == 0 {
                    progress_bar.print_progress_percent();
                    progress_bar.inc();
                }
            }
        });
    } else {
        for y in 0..image_height {
            for x in 0..image_width {
                let mut sum_texel_color: Color = Color::new(0.0, 0.0, 0.0, 1.0);
                for _aa in 0..camera.samples_per_pixel {
                    let ray: Ray = camera.get_ray(x, y);

                    let texel_color: Color = ray_color(&ray, camera.max_ray_per_pixel, &world);
                    sum_texel_color += texel_color;
                }
                write_color(image_string, sum_texel_color, camera.samples_per_pixel);

                if (x + y * image_width) % progress_bar.calc_increment() as i32 == 0 {
                    progress_bar.print_progress_percent();
                    progress_bar.inc();
                }
            }
        }
    }

    assert!(progress_bar.is_finished());
}

fn write_color(accum_string_file: &mut String, texel_color: Color, samples_per_pixel: i32) {
    let scale_factor = 1.0 / samples_per_pixel as f32;
    let mut scaled_texel_color = Color {
        color: palette::rgb::Rgb {
            red: texel_color.red * scale_factor,
            green: texel_color.green * scale_factor,
            blue: texel_color.blue * scale_factor,
            standard: std::marker::PhantomData,
        },
        alpha: 1.0,
    };

    let gamma_corrected: Srgba = scaled_texel_color.clamp().into();
    let texel_color_u8 = color_to_u8_srgba(&gamma_corrected);

    let ir = texel_color_u8[0];
    let ig = texel_color_u8[1];
    let ib = texel_color_u8[2];

    *accum_string_file += &ir.to_string();
    *accum_string_file += &' '.to_string();
    *accum_string_file += &ig.to_string();
    *accum_string_file += &' '.to_string();
    *accum_string_file += &ib.to_string();
    *accum_string_file += &'\n'.to_string();
}

fn ray_color(ray: &Ray, depth: i32, world: &HittableList) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0, 0.0);
    }

    if let Some(hit_result) = world.hit_all(
        ray,
        Interval {
            min: 0.0001,
            max: f32::INFINITY,
        },
    ) {
        let mut scattererd: Ray = Ray::default();
        let mut diffuse: Color = Color::new(0.0, 0.0, 0.0, 1.0);
        let mut emissive: Color = Color::new(0.0, 0.0, 0.0, 1.0);
        if scatter(
            hit_result.material_id,
            ray,
            &hit_result,
            &mut diffuse,
            &mut emissive,
            &mut scattererd,
        ) {
            return emissive + diffuse * ray_color(&scattererd, depth - 1, world);
        }
        return diffuse + emissive;
    }

    // BG
    let unit_dir = ray.direction.normalize();
    let a = (unit_dir.y + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0, 1.0) * a
}
