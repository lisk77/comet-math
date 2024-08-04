use std::ops::{Add, Sub, Mul, Div};
use crate::vector::{Vec2, Vec3, Vec4};

trait LinearTransformation {
    fn det(&self) -> f32;
}

// ##################################################
// #                   MATRIX 2D                    #
// ##################################################


/// Representation of a 2x2 Matrix in row major
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq)]
pub struct Mat2 {
    x00: f32,
    x01: f32,
    x10: f32,
    x11: f32,
}

impl Mat2 {
    pub fn new(x00: f32, x01: f32, x10: f32, x11: f32) -> Self {
        Self {
            x00: x00, x01: x01,
            x10: x10, x11: x11
        }
    }

    pub fn id() -> Self {
        Self { 
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

    pub fn getRow(&self, row: usize) -> Option<Vec2> {
        match row {
            0 => Some(Vec2 { x: self.x00, y: self.x01 }),
            1 => Some(Vec2 { x: self.x10, y: self.x11 }),
            _ => { println!("Row {} does not exist in the matrix!", row); return None; }
        }
    }

    pub fn set(&mut self, row: usize, col: usize, element: f32) {
        match (row, col) {
            (0,0) => self.x00 = element,
            (0,1) => self.x01 = element,
            (1,0) => self.x10 = element,
            (1,1) => self.x11 = element,
            _ => println!("There is no element ({}, {}) in the matrix!", row, col)        
        }
    }

    pub fn setRow(&mut self, row: usize, rowContent: Vec2) {
        match row {
            0 => { self.x00 = rowContent.x; self.x01 = rowContent.y; },
            1 => { self.x10 = rowContent.x; self.x11 = rowContent.y; },
            _ => println!("Row {} does not exist in the matrix!", row)
        }
    }

    pub fn det(&self) -> f32 {
        self.x00 * self.x11 - self.x01 * self.x10
    }

    pub fn transpose(&self) -> Self {
        Self {
            x00: self.x00, x01: self.x10,
            x10: self.x01, x11: self.x11
        }
    }

   pub fn swapRows(&mut self, row1: usize, row2: usize) {
        let tmp = self.getRow(row1).expect(format!("There is no row {} in the matrix!", row1).as_str());
        self.setRow(row1, self.getRow(row2).expect(format!("There is no row {} in the matrix!", row2).as_str()));
        self.setRow(row2, tmp);
   }
}

impl Add<Mat2> for Mat2 {
    type Output = Self;

    fn add(self, other: Mat2) -> Self {
        Self { 
            x00: self.x00 + other.x00, x01: self.x01 + other.x01,
            x10: self.x10 + other.x10, x11: self.x11 + other.x11 
        }
    }
}

impl Sub<Mat2> for Mat2 {
    type Output = Self;

    fn sub(self, other: Mat2) -> Self {
        Self { 
            x00: self.x00 - other.x00, x01: self.x01 - other.x01,
            x10: self.x10 - other.x10, x11: self.x11 - other.x11 
        }
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x00: self.x00 * other, x01: self.x01 * other,
            x10: self.x10 * other, x11: self.x11 * other
        }
    }
}

impl Div<f32> for Mat2 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x00: self.x00 / other, x01: self.x01 / other,
            x10: self.x10 / other, x11: self.x11 / other
        }
    }
}

// ##################################################
// #                   MATRIX 3D                    #
// ##################################################

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
        Self { 
            x00: x00, x01: x01, x02: x02, 
            x10: x10, x11: x11, x12: x12, 
            x20: x20, x21: x21, x22: x22 
        }
    }

    pub fn id() -> Self {
        Self { 
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

    pub fn getRow(&self, row: usize) -> Option<Vec3> {
        match row {
            0 => Some(Vec3 { x: self.x00, y: self.x01, z: self.x02 }),
            1 => Some(Vec3 { x: self.x10, y: self.x11, z: self.x12 }),
            2 => Some(Vec3 { x: self.x20, y: self.x21, z: self.x22 }),
            _ => None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, element: f32) {
        match (row, col) {
            (0,0) => self.x00 = element,
            (0,1) => self.x01 = element,
            (0,2) => self.x02 = element,
            (1,0) => self.x10 = element,
            (1,1) => self.x11 = element,
            (1,2) => self.x12 = element,
            (2,0) => self.x20 = element,
            (2,1) => self.x21 = element,
            (2,2) => self.x22 = element,
            _ => println!("There is no element ({}, {}) in the matrix!", row, col)        
        }
    }

    pub fn setRow(&mut self, row: usize, rowContent: Vec3) {
        match row {
            0 => { self.x00 = rowContent.x; self.x01 = rowContent.y; self.x02 = rowContent.z; },
            1 => { self.x10 = rowContent.x; self.x11 = rowContent.y; self.x12 = rowContent.z; },
            2 => { self.x20 = rowContent.x; self.x21 = rowContent.y; self.x22 = rowContent.z; }
            _ => println!("Row {} does not exist in the matrix!", row)
        }
    }
    
    pub fn det(&self) -> f32 {
        self.x00 * self.x11 * self.x22 + self.x01 * self.x12 * self.x20 + self.x02 * self.x10 * self.x21 - self.x02 * self.x11 * self.x20 - self.x01 * self.x10 * self.x22 - self.x00 * self.x12 * self.x21
    }

    pub fn transpose(&self) -> Self {
        Self {
            x00: self.x00, x01: self.x10, x02: self.x20,
            x10: self.x01, x11: self.x11, x12: self.x21,
            x20: self.x02, x21: self.x12, x22: self.x22
        }
    }

    pub fn swapRows(&mut self, row1: usize, row2: usize) {
        let tmp = self.getRow(row1).expect(format!("There is no row {} in the matrix!", row1).as_str());
        self.setRow(row1, self.getRow(row2).expect(format!("There is no row {} in the matrix!", row2).as_str()));
        self.setRow(row2, tmp);
    }
}

// ##################################################
// #                   MATRIX 4D                    #
// ##################################################

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
        Self { 
            x00: x00, x01: x01, x02: x02, x03: x03, 
            x10: x10, x11: x11, x12: x12, x13: x13, 
            x20: x20, x21: x21, x22: x22, x23: x23, 
            x30: x30, x31: x31, x32: x32, x33: x33 
        }
    }

    pub fn id() -> Self {
        Self { 
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

    pub fn getRow(&self, row: usize) -> Option<Vec4> {
        match row {
            0 => Some(Vec4 { x: self.x00, y: self.x01, z: self.x02, w: self.x03 }),
            1 => Some(Vec4 { x: self.x10, y: self.x11, z: self.x12, w: self.x13 }),
            2 => Some(Vec4 { x: self.x20, y: self.x21, z: self.x22, w: self.x23 }),
            3 => Some(Vec4 { x: self.x30, y: self.x31, z: self.x32, w: self.x33 }),
            _ => None
        }
    }
    
    pub fn set(&mut self, row: usize, col: usize, element: f32) {
        match (row, col) {
            (0,0) => self.x00 = element,
            (0,1) => self.x01 = element,
            (0,2) => self.x02 = element,
            (0,3) => self.x03 = element,
            (1,0) => self.x10 = element,
            (1,1) => self.x11 = element,
            (1,2) => self.x12 = element,
            (1,3) => self.x13 = element,
            (2,0) => self.x20 = element,
            (2,1) => self.x21 = element,
            (2,2) => self.x22 = element,
            (2,3) => self.x23 = element,
            (3,0) => self.x30 = element,
            (3,1) => self.x31 = element,
            (3,2) => self.x32 = element,
            (3,3) => self.x33 = element,
            _ => println!("There is no element ({}, {}) in the matrix!", row, col)        
        }
    }

    pub fn setRow(&mut self, row: usize, rowContent: Vec4) {
        match row {
            0 => { self.x00 = rowContent.x; self.x01 = rowContent.y; self.x02 = rowContent.z; self.x03 = rowContent.w; },
            1 => { self.x10 = rowContent.x; self.x11 = rowContent.y; self.x12 = rowContent.z; self.x13 = rowContent.w; },
            2 => { self.x20 = rowContent.x; self.x21 = rowContent.y; self.x22 = rowContent.z; self.x23 = rowContent.w; },
            3 => { self.x30 = rowContent.x; self.x31 = rowContent.y; self.x32 = rowContent.z; self.x33 = rowContent.w; }
            _ => println!("Row {} does not exist in the matrix!", row)
        }
    }
    
    pub fn det(&self) -> f32 {
        self.x00 * (self.x11 * (self.x22* self.x33 - self.x23 * self.x32) - self.x21 * (self.x12 * self.x33 - self.x13 * self.x32) + self.x31 * (self.x12 * self.x23 - self.x13 * self.x22)) - self.x10 * (self.x01 * (self.x22* self.x33 - self.x23 * self.x32) - self.x21 * (self.x02 * self.x33 - self.x32 * self.x03) + self.x31 * (self.x02 * self.x23 - self.x22 * self.x03)) + self.x20 * ( self.x01 * (self.x12 * self.x33 - self.x13 * self.x32) - self.x11 * (self.x02 * self.x33 - self.x03 * self.x32) + self.x31 * (self.x02 * self.x13 - self.x03 * self.x12)) - self.x30 * (self.x01 * (self.x12 * self.x23 - self.x22 * self.x13) - self.x11 * (self.x02 * self.x23 - self.x22 * self.x03) + self.x21 * (self.x02 * self.x13 - self.x03 * self.x12))
    }

    pub fn transpose(&self) -> Self {
        Self { 
            x00: self.x00, x01: self.x10, x02: self.x20, x03: self.x30, 
            x10: self.x01, x11: self.x11, x12: self.x21, x13: self.x31, 
            x20: self.x02, x21: self.x12, x22: self.x22, x23: self.x32, 
            x30: self.x03, x31: self.x13, x32: self.x23, x33: self.x33 
        }
    }
}

// ##################################################
// #              MATRIX FUNCTIONS                  #
// ##################################################

pub fn det<T: LinearTransformation>(mat: &T) -> f32 {
    mat.det()
}
