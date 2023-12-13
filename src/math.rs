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

    pub fn rand<T>() -> T
    where
        Standard: Distribution<T>,
    {
        let mut rng = rand::thread_rng();
        let r: T = rng.gen::<T>();
        return r;
    }

    pub fn rand_range<T, R>(range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        let mut rng = rand::thread_rng();
        let r: T = rng.gen_range::<T, R>(range);
        return r;
    }

    pub fn rand_vec3() -> Vec3 {
        Vec3::new(rand::<f32>(), rand::<f32>(), rand::<f32>())
    }

    pub fn rand_vec3_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            rand_range(min..max),
            rand_range(min..max),
            rand_range(min..max),
        )
    }

    pub fn rand_sphere_vec3() -> Vec3 {
        while true {
            let p = rand_vec3_range(-1.0, 1.0);
            if (p.length_squared() < 1.0) {
                return p;
            }
        }

        return Vec3::new(0.0, 0.0, 0.0);
    }

    pub fn unit_vector(x: Vec3) -> Vec3 {
        x.normalize()
    }
    pub fn rand_unit_vector() -> Vec3 {
        unit_vector(rand_sphere_vec3())
    }

    pub fn rand_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = rand_unit_vector();

        let hemisphere_unit_vector = if (on_unit_sphere.dot(normal) > 0.0) {
            on_unit_sphere
        } else {
            -on_unit_sphere
        };

        hemisphere_unit_vector
    }
}
