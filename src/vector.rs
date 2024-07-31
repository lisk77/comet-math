pub trait InnerSpace {
    fn dot(&self, other: &Self) -> f32;
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
}

impl InnerSpace for Vec2 {
    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl InnerSpace for Vec3 {
    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl InnerSpace for Vec4 {
    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

pub fn dot<T: InnerSpace>(v1: &T, v2: &T) -> f32 {
    v1.dot(v2)
}

