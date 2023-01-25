
use crate::Vec3;
use crate::Point;

#[derive (Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction 
    }
}

pub fn new_ray(origin: Point, direction: Vec3) -> Ray {
    Ray { origin, direction, }
}