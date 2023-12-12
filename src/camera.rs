use std::{f32::INFINITY, fs::File, io::Write};

use palette::Clamp;

use crate::{
    color::{color::color_to_u8, Color},
    interval::Interval,
    math::{
        math::{rand_f32, rand_f32_range, rand_on_hemisphere},
        Vec3, Vec4,
    },
    progress_bar::ProgressBar,
    ray::{HitResult, HittableList, Ray},
};

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec4,

    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_ray_per_pixel: i32,

    image_height: i32,
    image_size: [i32; 2],

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
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
            samples_per_pixel: 1,
            max_ray_per_pixel: 3,
            image_height: 0,
            image_size: [0, 0],
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
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

        let mut camera_center = self.position;
        let mut focal_length = 1.0;

        let viewport_u = Vec3::new(viewport_width as f32, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height as f32, 0.0);
        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut sum_texel_color: Color = Color::new(0.0, 0.0, 0.0, 1.0);
                for aa in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(x, y);

                    let texel_color: Color = Self::ray_color(&ray, self.max_ray_per_pixel, &world);
                    sum_texel_color += texel_color;
                }
                Self::write_color(&mut image_ppm, sum_texel_color, self.samples_per_pixel);

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
        let texel_color_u8 = color_to_u8(&scaled_texel_color.clamp());

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
        if (depth <= 0) {
            return Color::new(0.0, 0.0, 0.0, 0.0);
        }

        let mut hit_result: HitResult = HitResult::default();
        if world.hit(
            ray,
            Interval {
                min: 0.0001,
                max: INFINITY,
            },
            &mut hit_result,
        ) {
            let direction = rand_on_hemisphere(hit_result.normal);
            let col = Self::ray_color(&Ray::new(hit_result.location, direction), depth - 1, world);
            return Color::new(col.red * 0.5, col.green * 0.5, col.blue * 0.5, 1.0);
        }

        // BG
        let unit_dir = ray.direction.normalize();
        let a = (unit_dir.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0, 1.0) * a
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {
        let pixel_center: Vec3 =
            self.pixel00_loc + (self.pixel_delta_u * x as f32) + (self.pixel_delta_v * y as f32);
        let pixel_rand_offset = if self.samples_per_pixel > 1 {
            self.pixel_sample_square()
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        };
        let pixel_sample = pixel_center + pixel_rand_offset;

        let ray_origin = self.position;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = rand_f32_range(-0.5, 0.5);
        let py = rand_f32_range(-0.5, 0.5);
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
