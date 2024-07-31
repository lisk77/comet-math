/// Representation of a 2x2 Matrix
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat2 {
    pub x00: f32,
    pub x01: f32,
    pub x10: f32,
    pub x11: f32,
}

/// Representation of a 3x3 Matrix
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat3 {
    pub x00: f32,
    pub x01: f32,
    pub x02: f32,
    pub x10: f32,
    pub x11: f32,
    pub x12: f32,
    pub x20: f32,
    pub x21: f32,
    pub x22: f32
}

/// Representation of a 4x4 Matrix
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat4 {
    pub x00: f32,
    pub x01: f32,
    pub x02: f32,
    pub x03: f32,
    pub x10: f32,
    pub x11: f32,
    pub x12: f32,
    pub x13: f32,
    pub x20: f32,
    pub x21: f32,
    pub x22: f32,
    pub x30: f32,
    pub x31: f32,
    pub x32: f32,
    pub x33: f32
}

