use std::ops;

type Vec3_InnerType = f64;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: Vec3_InnerType,
    pub y: Vec3_InnerType,
    pub z: Vec3_InnerType,
}

impl Vec3 {
    pub fn new(x: Vec3_InnerType, y: Vec3_InnerType, z: Vec3_InnerType) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn dot(&self, other: Vec3) -> Vec3_InnerType {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y,
        }
    }

    pub fn len_squared(&self) -> Vec3_InnerType {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> Vec3_InnerType {
        self.len_squared().sqrt()
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

impl ops::Add<Vec3_InnerType> for Vec3 {
    type Output = Self;
    fn add(self, _rhs: Vec3_InnerType) -> Self {
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

impl ops::Sub<Vec3_InnerType> for Vec3 {
    type Output = Self;
    fn sub(self, _rhs: Vec3_InnerType) -> Self {
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

impl ops::Mul<Vec3_InnerType> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: Vec3_InnerType) -> Self {
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

impl ops::Div<Vec3_InnerType> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: Vec3_InnerType) -> Self {
        self.div(Vec3::new(_rhs, _rhs, _rhs))
    }
}

pub struct Vec4 {
    pub x: Vec3_InnerType,
    pub y: Vec3_InnerType,
    pub z: Vec3_InnerType,
    pub w: Vec3_InnerType,
}

impl Vec4 {
    pub fn new(x: Vec3_InnerType, y: Vec3_InnerType, z: Vec3_InnerType, w: Vec3_InnerType) -> Self {
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

impl ops::Add<Vec3_InnerType> for Vec4 {
    type Output = Self;
    fn add(self, _rhs: Vec3_InnerType) -> Self {
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

impl ops::Sub<Vec3_InnerType> for Vec4 {
    type Output = Self;
    fn sub(self, _rhs: Vec3_InnerType) -> Self {
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

impl ops::Mul<Vec3_InnerType> for Vec4 {
    type Output = Self;
    fn mul(self, _rhs: Vec3_InnerType) -> Self {
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

impl ops::Div<Vec3_InnerType> for Vec4 {
    type Output = Self;
    fn div(self, _rhs: Vec3_InnerType) -> Self {
        self.div(Vec4::new(_rhs, _rhs, _rhs, _rhs))
    }
}

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
