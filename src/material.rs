use palette::white_point::E;

use crate::{
    color::Color,
    math::math::{near_zero_vec3, reflect, refract, schlick, unit_vector},
    random::*,
    ray::{HitResult, Ray},
};

pub const MATERIAL_DEFAULT: i32 = 0;
pub const MATERIAL_LAMBERTIAN: i32 = 1;
pub const MATERIAL_METAL: i32 = 2;
pub const MATERIAL_DIELECTRIC: i32 = 3;
pub const MATERIAL_NUM: i32 = 4;

pub fn scatter(
    material_id: i32,
    ray: &Ray,
    hit_result: &HitResult,
    diffuse: &mut Color,
    emissive: &mut Color,
    scattered_ray: &mut Ray,
) -> bool {
    let surface_albedo = hit_result.surface.albedo;
    let surface_emissive = hit_result.surface.emissive;

    *emissive = surface_emissive;

    if material_id == MATERIAL_LAMBERTIAN || material_id == MATERIAL_DEFAULT {
        let mut scatter_direction = hit_result.normal + rand_unit_vector();
        if near_zero_vec3(scatter_direction) {
            scatter_direction = hit_result.normal;
        }
        *scattered_ray = Ray::new(hit_result.location, scatter_direction);
        *diffuse = surface_albedo;
        return true;
    } else if material_id == MATERIAL_METAL {
        let fuzz_amount: f32 = 0.0;
        let reflected = reflect(ray.direction.normalize(), hit_result.normal);
        *scattered_ray = Ray::new(
            hit_result.location,
            reflected + fuzz_amount * rand_unit_vector(),
        );
        *diffuse = surface_albedo;
        return scattered_ray.direction.dot(hit_result.normal) > 0.0;
    } else if material_id == MATERIAL_DIELECTRIC {
        let ir = hit_result.surface.ir;
        let refraction_ratio = if hit_result.front_face.unwrap() {
            1.0 / ir
        } else {
            ir
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction.dot(hit_result.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || schlick(cos_theta, refraction_ratio) > rand() {
            reflect(unit_direction, hit_result.normal)
        } else {
            let refracted = refract(unit_direction, hit_result.normal, refraction_ratio);
            refracted
        };

        // todo: Tint with color
        *diffuse = Color::new(1.0, 1.0, 1.0, 1.0);
        *scattered_ray = Ray::new(hit_result.location, direction);
        return true;
    }

    return false;
}
