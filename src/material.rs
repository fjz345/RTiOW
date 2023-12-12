use crate::{
    color::Color,
    math::math::{near_zero_vec3, rand_unit_vector, reflect},
    ray::{HitResult, Ray},
};

const MATERIAL_DEFAULT: i32 = 0;
const MATERIAL_LAMBERTIAN: i32 = 1;
const MATERIAL_METAL: i32 = 2;

pub fn scatter(
    material_id: i32,
    ray: &Ray,
    hit_result: &HitResult,
    attenuation: &mut Color,
    scattered_ray: &mut Ray,
) -> bool {
    let albedo = Color::new(0.5, 0.5, 0.5, 0.0);

    if material_id == MATERIAL_LAMBERTIAN || material_id == MATERIAL_DEFAULT {
        let mut scatter_direction = hit_result.normal + rand_unit_vector();
        if near_zero_vec3(scatter_direction) {
            scatter_direction = hit_result.normal;
        }
        *scattered_ray = Ray::new(hit_result.location, scatter_direction);
        *attenuation = albedo;
        return true;
    } else if (material_id == MATERIAL_METAL) {
        let reflected = reflect(ray.direction.normalize(), hit_result.normal);
        *scattered_ray = Ray::new(hit_result.location, reflected);
        *attenuation = albedo;
        return true;
    }

    return false;
}
