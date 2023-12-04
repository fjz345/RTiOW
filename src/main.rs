use std::{fs::File, io::Write, process::Output};

use crate::{math::Color::Color, progress_bar::ProgressBar};

mod math;
mod progress_bar;
mod ray;

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i64 = 800;
    let image_height: i64 = ((image_width as f64 / aspect_ratio) as i64).max(1);
    let image_size = (image_width, image_height);

    let viewport_aspectratio = (image_width as f64) / (image_height as f64);
    let viewport_height = 2.0;
    let viewport_width = viewport_height * viewport_aspectratio;
    let viewport_size = (viewport_width, viewport_height);

    let mut image_ppm: String = String::new();
    image_ppm += &format!("P3\n{} {}\n255\n", image_width, image_height).to_string();

    let mut progress_bar: ProgressBar = ProgressBar::new((image_width * image_height) as f64, 20);
    for y in 0..image_height {
        for x in 0..image_width {
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = 0.0_f64;

            let texel_color: Color = Color::new(r, g, b, 1.0);
            let texel_color_u8 = texel_color.to_u8();

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
