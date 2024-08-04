use crate::vector::{Vec2, Vec3};

pub struct Bezier2 {
    points: Vec<Vec2>,
    degree: i8
}

impl Bezier2 {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points: points.clone(), degree: points.len() as i8 }
    }
}

pub struct Bezier3 {
    points: Vec<Vec3>,
    degree: i8
}

impl Bezier3 {
    pub fn new(points: Vec<Vec3>) -> Self {
        Self { points: points.clone(), degree: points.len() as i8 }
    }
}
