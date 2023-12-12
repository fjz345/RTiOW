pub use glam::*;

pub mod math {
    use rand::thread_rng;
    use rand::Rng;

    const PI: f64 = 3.1415926535897932385;

    pub fn deg_to_rad(deg: f64) -> f64 {
        deg * PI / 180.0
    }

    pub fn rad_to_deg(rad: f64) -> f64 {
        (rad * 180.0) / PI
    }

    pub fn rand_f32() -> f32 {
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen();
        return r;
    }
    pub fn rand_f32_range(min: f32, max: f32) -> f32 {
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen_range(min..max);
        return r;
    }
}
