/// Representation of a 2x2 Matrix in row major
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat2 {
    x00: f32,
    x01: f32,
    x10: f32,
    x11: f32,
}

impl Mat2 {
    pub fn new(x00: f32, x01: f32, x10: f32, x11: f32) -> Self {
        Mat2 {
            x00: x00, x01: x01,
            x10: x10, x11: x11
        }
    }

    pub fn id() -> Self {
        Mat2 { 
            x00: 1.0, x01: 0.0, 
            x10: 0.0, x11: 1.0 
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        match (row, col) {
            (0, 0) => Some(self.x00),
            (0, 1) => Some(self.x01),
            (1, 0) => Some(self.x10),
            (1, 1) => Some(self.x11),
            _ => None, 
        }
    }

    pub fn det(&self) -> f32 {
        self.x00 * self.x11 - self.x01 * self.x10
    }
}

/// Representation of a 3x3 Matrix
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat3 {
    x00: f32,
    x01: f32,
    x02: f32,
    x10: f32,
    x11: f32,
    x12: f32,
    x20: f32,
    x21: f32,
    x22: f32
}

impl Mat3 {
    pub fn new(x00: f32, x01: f32, x02: f32, x10: f32, x11: f32, x12: f32, x20: f32, x21: f32, x22: f32) -> Self {
        Mat3 { 
            x00: x00, x01: x01, x02: x02, 
            x10: x10, x11: x11, x12: x12, 
            x20: x20, x21: x21, x22: x22 
        }
    }

    pub fn id() -> Self {
        Mat3 { 
            x00: 1.0, x01: 0.0, x02: 0.0, 
            x10: 0.0, x11: 1.0, x12: 0.0, 
            x20: 0.0, x21: 0.0, x22: 1.0 
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        match (row, col) {
            (0, 0) => Some(self.x00),
            (0, 1) => Some(self.x01),
            (0, 2) => Some(self.x02),
            (1, 0) => Some(self.x10),
            (1, 1) => Some(self.x11),
            (1, 2) => Some(self.x12),
            (2, 0) => Some(self.x20),
            (2, 1) => Some(self.x21),
            (2, 2) => Some(self.x22),
            _ => None, 
        }
    }

    pub fn det(&self) -> f32 {
        self.x00 * self.x11 * self.x22 + self.x01 * self.x12 * self.x20 + self.x02 * self.x10 * self.x21 - self.x02 * self.x11 * self.x20 - self.x01 * self.x10 * self.x22 - self.x00 * self.x12 * self.x21
    }
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
    pub x23: f32,
    pub x30: f32,
    pub x31: f32,
    pub x32: f32,
    pub x33: f32
}

impl Mat4 {
    pub fn new(x00: f32, x01: f32,x02: f32,x03: f32,x10: f32,x11: f32,x12: f32,x13: f32,x20: f32,x21: f32,x22: f32,x23: f32,x30: f32, x31: f32, x32: f32,x33: f32) -> Self {
        Mat4 { 
            x00: x00, x01: x01, x02: x02, x03: x03, 
            x10: x10, x11: x11, x12: x12, x13: x13, 
            x20: x20, x21: x21, x22: x22, x23: x23, 
            x30: x30, x31: x31, x32: x32, x33: x33 
        }
    }

    pub fn id() -> Self {
        Mat4 { 
            x00: 1.0, x01: 0.0, x02: 0.0, x03: 0.0, 
            x10: 0.0, x11: 1.0, x12: 0.0, x13: 0.0, 
            x20: 0.0, x21: 0.0, x22: 1.0, x23: 0.0, 
            x30: 0.0, x31: 0.0, x32: 0.0, x33: 1.0 
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        match (row, col) {
            (0, 0) => Some(self.x00),
            (0, 1) => Some(self.x01),
            (0, 2) => Some(self.x02),
            (0, 3) => Some(self.x03),
            (1, 0) => Some(self.x10),
            (1, 1) => Some(self.x11),
            (1, 2) => Some(self.x12),
            (1, 3) => Some(self.x13),
            (2, 0) => Some(self.x20),
            (2, 1) => Some(self.x21),
            (2, 2) => Some(self.x22),
            (2, 3) => Some(self.x23),
            (3, 0) => Some(self.x30),
            (3, 1) => Some(self.x31),
            (3, 2) => Some(self.x32),
            (3, 3) => Some(self.x33),
            _ => None, 
        }
    }

    pub fn det(&self) -> f32 {
        self.x00 * (self.x11 * (self.x22* self.x33 - self.x23 * self.x32) - self.x21 * (self.x12 * self.x33 - self.x13 * self.x32) + self.x31 * (self.x12 * self.x23 - self.x13 * self.x22)) - self.x10 * (self.x01 * (self.x22* self.x33 - self.x23 * self.x32) - self.x21 * (self.x02 * self.x33 - self.x32 * self.x03) + self.x31 * (self.x02 * self.x23 - self.x22 * self.x03)) + self.x20 * ( self.x01 * (self.x12 * self.x33 - self.x13 * self.x32) - self.x11 * (self.x02 * self.x33 - self.x03 * self.x32) + self.x31 * (self.x02 * self.x13 - self.x03 * self.x12)) - self.x30 * (self.x01 * (self.x12 * self.x23 - self.x22 * self.x13) - self.x11 * (self.x02 * self.x23 - self.x22 * self.x03) + self.x21 * (self.x02 * self.x13 - self.x03 * self.x12))
    }
}
