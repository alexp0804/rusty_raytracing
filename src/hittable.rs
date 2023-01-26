
use crate::Vec3;
use crate::Ray;
use crate::Point;

#[derive (Copy, Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point, normal: Vec3, t: f32, front_face: bool) -> HitRecord {
        HitRecord { p, normal, t, front_face }
    }
    pub fn new_empty() -> HitRecord {
        HitRecord { p: Point::origin(), normal: Vec3::origin(), t: 0.0, front_face: false }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = (outward_normal * ray.direction) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -1.0 * outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
