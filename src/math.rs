pub use glam::*;

pub mod math {
    use glam::Vec3;
    use rand::distributions::uniform::SampleRange;
    use rand::distributions::uniform::SampleUniform;
    use rand::distributions::Distribution;
    use rand::distributions::Standard;
    use rand::thread_rng;
    use rand::Rng;

    const PI: f64 = 3.1415926535897932385;

    pub fn deg_to_rad(deg: f64) -> f64 {
        deg * PI / 180.0
    }

    pub fn rad_to_deg(rad: f64) -> f64 {
        (rad * 180.0) / PI
    }

    pub fn reflect(vec: Vec3, n: Vec3) -> Vec3 {
        vec - 2.0 * vec.dot(n) * n
    }

    pub fn near_zero_vec3(vec: Vec3) -> bool {
        const s: f32 = 1e-8;
        vec[0].abs() < s && vec[1].abs() < s && vec[2].abs() < s
    }

    pub fn unit_vector(x: Vec3) -> Vec3 {
        x.normalize()
    }
}
