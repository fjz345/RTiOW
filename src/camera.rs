use std::{default, f32::INFINITY, fs::File, io::Write};

use palette::{Clamp, Srgb, Srgba};

use crate::{
    color::{color::*, Color},
    interval::*,
    material::*,
    math::{math::*, *},
    progress_bar::ProgressBar,
    random::*,
    ray::*,
};

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec4,
    pub fov: f32, // Deg

    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_ray_per_pixel: i32,

    image_height: i32,
    image_size: [i32; 2],

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,

    camera_mat: Mat3,

    pub defocus_angle: f32, // Variation angle of rays through each pixel
    pub focus_dist: f32,    // distance from camera lookfrom point to plane of perfect focus

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
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
            fov: 90.0,
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 1,
            max_ray_per_pixel: 3,
            image_height: 0,
            image_size: [0, 0],
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            camera_mat: Mat3::from_cols(
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vec3::new(1.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn look_at(&mut self, world_location: Vec3, up_vector: Vec3) {
        let forward: Vec3 = (self.position - world_location).normalize();
        assert_ne!(forward, up_vector);
        let right = up_vector.normalize().cross(forward);
        let up = forward.cross(right);

        self.camera_mat = Mat3::from_cols(right, up, forward);
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        let mut progress_bar: ProgressBar =
            ProgressBar::new((self.image_width * self.image_height) as f64, 20);

        let mut image_ppm: String = String::new();
        image_ppm += &format!("P3\n{} {}\n255\n", self.image_width, self.image_height).to_string();

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

        let theta = deg_to_rad(self.fov as f64);
        let h = (theta / 2.0).tan();
        let viewport_aspectratio = (self.image_width as f64) / (self.image_height as f64);
        let viewport_height: f32 = (2.0 * h * self.focus_dist as f64) as f32;
        let viewport_width: f32 = (viewport_height * viewport_aspectratio as f32) as f32;

        let viewport_u = viewport_width * self.camera_mat.x_axis;
        let viewport_v = viewport_height * -self.camera_mat.y_axis;
        let viewport_upper_left = self.position
            - self.camera_mat.z_axis * self.focus_dist as f32
            - viewport_u / 2.0
            - viewport_v / 2.0;

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius: f32 =
            (self.focus_dist * deg_to_rad(self.defocus_angle as f64 * 0.5) as f32).tan();
        self.defocus_disk_u = self.camera_mat.x_axis * defocus_radius;
        self.defocus_disk_u = self.camera_mat.y_axis * defocus_radius;
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
                max: INFINITY,
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
                return emissive + diffuse * Self::ray_color(&scattererd, depth - 1, world);
            }
            return diffuse + emissive;
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

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.position
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = rand_range(-0.5..0.5);
        let py = rand_range(-0.5..0.5);
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = rand_disc_vec2();
        return self.position + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v);
    }
}
