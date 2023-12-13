use crate::{
    color::Color,
    math::math::{near_zero_vec3, reflect},
    random::*,
    ray::{HitResult, Ray},
};

pub const MATERIAL_DEFAULT: i32 = 0;
pub const MATERIAL_LAMBERTIAN: i32 = 1;
pub const MATERIAL_METAL: i32 = 2;
pub const MATERIAL_NUM: i32 = 3;

pub fn scatter(
    material_id: i32,
    ray: &Ray,
    hit_result: &HitResult,
    attenuation: &mut Color,
    scattered_ray: &mut Ray,
) -> bool {
    let albedo = hit_result.surface.albedo;
    let emissive = hit_result.surface.emissive;

    if material_id == MATERIAL_LAMBERTIAN || material_id == MATERIAL_DEFAULT {
        let mut scatter_direction = hit_result.normal + rand_unit_vector();
        if near_zero_vec3(scatter_direction) {
            scatter_direction = hit_result.normal;
        }
        *scattered_ray = Ray::new(hit_result.location, scatter_direction);
        *attenuation = albedo;
        return true;
    } else if (material_id == MATERIAL_METAL) {
        let fuzz_amount: f32 = 0.0;
        let reflected = reflect(ray.direction.normalize(), hit_result.normal);
        *scattered_ray = Ray::new(
            hit_result.location,
            reflected + fuzz_amount * rand_unit_vector(),
        );
        *attenuation = albedo;
        return scattered_ray.direction.dot(hit_result.normal) > 0.0;
    }

    return false;
}
