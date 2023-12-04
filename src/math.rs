use std::ops;

type Vec_InnerType = f64;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: Vec_InnerType,
    pub y: Vec_InnerType,
}

impl From<Vec3> for Vec2 {
    fn from(vec: Vec3) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl Vec2 {
    pub fn new(x: Vec_InnerType, y: Vec_InnerType) -> Self {
        Vec2 { x: x, y: y }
    }

    pub fn dot(&self, other: Vec2) -> Vec_InnerType {
        self.x * other.x + self.y * other.y
    }

    pub fn len_squared(&self) -> Vec_InnerType {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> Vec_InnerType {
        self.len_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec2 {
        *self / self.len()
    }

    pub fn equal(&self, other: &Self, threshold: Option<f64>) -> bool {
        let threshold_value = threshold.unwrap_or(0.001);
        (*self - *other).x.abs() >= threshold_value && (*self - *other).y.abs() >= threshold_value
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, _rhs: Vec2) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Add<Vec_InnerType> for Vec2 {
    type Output = Self;
    fn add(self, _rhs: Vec_InnerType) -> Self {
        self.add(Vec2::new(_rhs, _rhs))
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, _rhs: Vec2) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl ops::Sub<Vec_InnerType> for Vec2 {
    type Output = Self;
    fn sub(self, _rhs: Vec_InnerType) -> Self {
        self.sub(Vec2::new(_rhs, _rhs))
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Self;
    fn mul(self, _rhs: Vec2) -> Self {
        Self {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        }
    }
}

impl ops::Mul<Vec_InnerType> for Vec2 {
    type Output = Self;
    fn mul(self, _rhs: Vec_InnerType) -> Self {
        self.mul(Vec2::new(_rhs, _rhs))
    }
}

impl ops::Div<Vec2> for Vec2 {
    type Output = Self;
    fn div(self, _rhs: Vec2) -> Self {
        Self {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
        }
    }
}

impl ops::Div<Vec_InnerType> for Vec2 {
    type Output = Self;
    fn div(self, _rhs: Vec_InnerType) -> Self {
        self.div(Vec2::new(_rhs, _rhs))
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: Vec_InnerType,
    pub y: Vec_InnerType,
    pub z: Vec_InnerType,
}

impl Vec3 {
    pub fn new(x: Vec_InnerType, y: Vec_InnerType, z: Vec_InnerType) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn dot(&self, other: Vec3) -> Vec_InnerType {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y,
        }
    }

    pub fn len_squared(&self) -> Vec_InnerType {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> Vec_InnerType {
        self.len_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        *self / self.len().max(0.0001)
    }

    pub fn equal(&self, other: &Self, threshold: Option<f64>) -> bool {
        let threshold_value = threshold.unwrap_or(0.001);
        (*self - *other).x.abs() >= threshold_value
            && (*self - *other).y.abs() >= threshold_value
            && (*self - *other).z.abs() >= threshold_value
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, _rhs: Vec3) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Add<Vec_InnerType> for Vec3 {
    type Output = Self;
    fn add(self, _rhs: Vec_InnerType) -> Self {
        self.add(Vec3::new(_rhs, _rhs, _rhs))
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, _rhs: Vec3) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<Vec_InnerType> for Vec3 {
    type Output = Self;
    fn sub(self, _rhs: Vec_InnerType) -> Self {
        self.sub(Vec3::new(_rhs, _rhs, _rhs))
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: Vec3) -> Self {
        Self {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl ops::Mul<Vec_InnerType> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: Vec_InnerType) -> Self {
        self.mul(Vec3::new(_rhs, _rhs, _rhs))
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: Vec3) -> Self {
        Self {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        }
    }
}

impl ops::Div<Vec_InnerType> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: Vec_InnerType) -> Self {
        self.div(Vec3::new(_rhs, _rhs, _rhs))
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: Vec_InnerType,
    pub y: Vec_InnerType,
    pub z: Vec_InnerType,
    pub w: Vec_InnerType,
}

impl Vec4 {
    pub fn new(x: Vec_InnerType, y: Vec_InnerType, z: Vec_InnerType, w: Vec_InnerType) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

impl ops::Add<Vec4> for Vec4 {
    type Output = Self;
    fn add(self, _rhs: Vec4) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

impl ops::Add<Vec_InnerType> for Vec4 {
    type Output = Self;
    fn add(self, _rhs: Vec_InnerType) -> Self {
        self.add(Vec4::new(_rhs, _rhs, _rhs, _rhs))
    }
}

impl ops::Sub<Vec4> for Vec4 {
    type Output = Self;
    fn sub(self, _rhs: Vec4) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}

impl ops::Sub<Vec_InnerType> for Vec4 {
    type Output = Self;
    fn sub(self, _rhs: Vec_InnerType) -> Self {
        self.sub(Vec4::new(_rhs, _rhs, _rhs, _rhs))
    }
}

impl ops::Mul<Vec4> for Vec4 {
    type Output = Self;
    fn mul(self, _rhs: Vec4) -> Self {
        Self {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
            w: self.w * _rhs.w,
        }
    }
}

impl ops::Mul<Vec_InnerType> for Vec4 {
    type Output = Self;
    fn mul(self, _rhs: Vec_InnerType) -> Self {
        self.mul(Vec4::new(_rhs, _rhs, _rhs, _rhs))
    }
}

impl ops::Div<Vec4> for Vec4 {
    type Output = Self;
    fn div(self, _rhs: Vec4) -> Self {
        Self {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
            w: self.w / _rhs.w,
        }
    }
}

impl ops::Div<Vec_InnerType> for Vec4 {
    type Output = Self;
    fn div(self, _rhs: Vec_InnerType) -> Self {
        self.div(Vec4::new(_rhs, _rhs, _rhs, _rhs))
    }
}

#[rustfmt::skip]
pub mod Color{
    use super::Vec4;

    pub type Color = Vec4;

    impl Color {
        pub fn to_u8(&self) -> [u8; 4] {
            let u8_vec: [u8; 4] = [
                (self.x * 256.0) as u8,
                (self.y * 256.0) as u8,
                (self.z * 256.0) as u8,
                (self.w * 256.0) as u8,
            ];
            u8_vec
        }
    }

    pub const BLACK: Color = Color {x: 0.0, y: 0.0, z: 0.0, w: 1.0};
    pub const WHITE: Color = Color {x: 1.0, y: 1.0, z: 1.0, w: 1.0};
    pub const TRANSPARENT: Color = Color {x:1.0, y: 1.0, z: 1.0,  w:0.0};

    pub const RED: Color = Color {x: 1.0, y:0.0, z: 0.0,  w:1.0};
    pub const GREEN: Color = Color {x: 0.0, y:1.0,  z:0.0,  w:1.0};
    pub const BLUE: Color = Color {x: 0.0, y:0.0, z: 1.0, w: 1.0};
    pub const YELLOW: Color = Color {x: 1.0, y:1.0, z: 0.0, w: 1.0};
    pub const PURPLE: Color = Color {x: 1.0, y:0.0,  z:1.0,  w:1.0};
    pub const TEAL: Color = Color {x: 0.0, y:1.0, z: 1.0,  w:1.0};
    pub const PINK: Color = Color {x: 1.0, y:0.0,  z:0.672,  w:1.0};
    pub const ORANGE: Color = Color {x: 1.0, y:0.348, z: 0.0, w: 10.0};

    pub const DARK_RED: Color = Color {x: 0.37, y:0.1, z: 0.1, w: 1.0};
    pub const DARK_GREEN: Color = Color {x: 0.1, y:0.37,  z:0.1, w: 1.0};
    pub const DARK_BLUE: Color = Color {x: 0.1, y:0.1,  z:0.37, w: 1.0};
    pub const DARK_YELLOW: Color = Color {x: 0.37, y:0.37, z: 0.1, w: 1.0};
    pub const DARK_PURPLE: Color = Color {x: 0.37, y:0.1, z: 0.37, w: 1.0};
    pub const DARK_TEAL: Color = Color {x: 0.1, y:0.37, z: 0.37,  w:1.0};
    pub const DARK_PINK: Color = Color {x: 0.37,y: 0.1, z: 0.24,  w:1.0};
    pub const DARK_ORANGE: Color = Color {x: 0.37, y:0.18, z: 0.1,  w:1.0};
}
