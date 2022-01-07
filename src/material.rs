use super::*;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: Vec3, scattered: &Ray) -> bool;
}
