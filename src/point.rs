use crate::vector::{Vec2, Vec3};

pub struct Point2 {
    pub x: f32,
    pub y: f32
}

pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point2 {
    pub fn new(x: f32, y: f32) -> Self {
        Point2 { x, y }
    }

    pub fn fromVec(v: Vec2) -> Self {
        Point2 { x: v.x, y: v.y }
    }
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3 { x, y, z }
    }

    pub fn fromVec(v: Vec3) -> Self {
        Point3 { x: v.x, y: v.y, z: v.z }
    }
}
