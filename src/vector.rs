use std::ops::{Add, Sub, Mul, Div};

pub trait InnerSpace {
    fn dot(&self, other: &Self) -> f32;
    fn dist(&self, other: &Self) -> f32;
}

/// Representation of a 2D Vector
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
} 

impl Vec2 {
    pub const fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub const fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn normalize(&self) -> Self {
        let factor = 1.0/self.length();
        Vec2 { x: factor * self.x, y: factor * self.y }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f32) -> Vec2 {
        Vec2 { x: self.x * other, y: self.y * other }
    }
}

/// Representation of a 3D Vector
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn zero() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let factor = 1.0/self.length();
        Vec3 { x: factor * self.x, y: factor * self.y, z: factor * self.z }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }

}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

/// Representation of a 4D Vector
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const fn zero() -> Self {
        Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let factor = 1.0/self.length();
        Vec4 { x: factor * self.x, y: factor * self.y, z: factor * self.z, w: factor * self.w }
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: self.w + other.w }
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: self.w - other.w }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: f32) -> Vec4 {
        Vec4 { x: self.x * other, y: self.y * other, z: self.z * other, w: self.w * other }
    }
}

impl InnerSpace for Vec2 {
    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    fn dist(&self, other: &Self) -> f32 {
        Vec2 { x: other.x - self.x, y: other.y - self.y }.length()
    }
}

impl InnerSpace for Vec3 {
    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn dist(&self, other: &Self) -> f32 {
        Vec3 { x: other.x - self.x, y: other.y - self.y, z: other.z - self.z }.length()
    }
}

impl InnerSpace for Vec4 {
    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn dist(&self, other: &Self) -> f32 {
        Vec4 { x: other.x - self.x, y: other.y - self.y, z: other.z - self.z, w: other.w - self.w }.length()
    }
}

pub fn dot<T: InnerSpace>(v1: &T, v2: &T) -> f32 {
    v1.dot(v2)
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 { x: v1.y * v2.z - v1.z * v2.y, y: v1.z * v2.x - v1.x * v2.z, z: v1.x * v2.y - v2.y * v1.x }
}

pub fn dist<T: InnerSpace>(v1: &T, v2: &T) -> f32 {
    v1.dist(v2)
}
