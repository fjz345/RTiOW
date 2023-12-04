use crate::math::{Color::*, Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, dir: Vec3) -> Self {
        Self {
            origin: o,
            direction: dir,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self) -> Color {
        let unit_dir = self.direction.normalize();
        let a = (unit_dir.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0, 1.0) * a
    }
}
