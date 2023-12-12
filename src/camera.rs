use std::{f32::INFINITY, fs::File, io::Write};

use crate::{
    color::{color::color_to_u8, Color},
    interval::Interval,
    math::{Vec3, Vec4},
    progress_bar::ProgressBar,
    ray::{HitResult, HittableList, Ray},
};

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec4,

    pub aspect_ratio: f32,
    pub image_width: i32,

    image_height: i32,
    image_size: [i32; 2],
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec4::new(0.0, 0.0, 0.0, 0.0),
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            image_size: [0, 0],
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        let mut progress_bar: ProgressBar =
            ProgressBar::new((self.image_width * self.image_height) as f64, 20);

        let mut image_ppm: String = String::new();
        image_ppm += &format!("P3\n{} {}\n255\n", self.image_width, self.image_height).to_string();

        let viewport_aspectratio = (self.image_width as f64) / (self.image_height as f64);
        let viewport_height = 2.0;
        let viewport_width = viewport_height * viewport_aspectratio;
        let viewport_size: (f64, f64) = (viewport_width, viewport_height);

        let mut camera_center = Vec3::new(0.0, 0.0, 0.0);
        let mut focal_length = 1.0;

        let viewport_u = Vec3::new(viewport_width as f32, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height as f32, 0.0);

        let pixel_delta_u: Vec3 = viewport_u / self.image_width as f32;
        let pixel_delta_v: Vec3 = viewport_v / self.image_height as f32;

        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc: Vec3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let pixel_center: Vec3 =
                    pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
                let pixel_dir = (pixel_center - camera_center);

                let ray: Ray = Ray::new(pixel_center, pixel_dir);

                let texel_color: Color = Self::ray_color(&ray, &world);
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

                if (x + y * self.image_width) % progress_bar.calc_increment() as i32 == 0 {
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

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f32 / self.aspect_ratio) as i32).max(1);
        self.image_size = [self.image_width, self.image_height];
    }

    fn ray_color(ray: &Ray, world: &HittableList) -> Color {
        let mut hit_result: HitResult = HitResult::default();
        if world.hit(
            ray,
            Interval {
                min: 0.0,
                max: INFINITY,
            },
            &mut hit_result,
        ) {
            let col = (hit_result.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
            return Color::new(col.x, col.y, col.z, 1.0);
        }

        // BG
        let unit_dir = ray.direction.normalize();
        let a = (unit_dir.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0, 1.0) * a
    }
}
