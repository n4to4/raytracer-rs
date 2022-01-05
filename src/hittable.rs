use super::*;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}
