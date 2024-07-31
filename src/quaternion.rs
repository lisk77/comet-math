use vector::Vec3;

/// Representation of a quaternion in scalar/vector form
pub struct Quat {
    pub s: f32,
    pub v: Vec3
}

impl Quat {
    pub const fn zero() -> Self {
        Quat { s: 0.0, v: Vec3 { x: 0.0, y: 0.0, z: 0.0 } }
    }
    pub const fn new(s: f32, v: Vec3) -> Self {
        Quat { s, v }
    } 
}
