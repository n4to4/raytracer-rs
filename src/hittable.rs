use super::*;
use std::rc::Rc;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new_with(
        r: &Ray,
        p: Vec3,
        t: f64,
        outward_normal: Vec3,
        mat_ptr: Rc<dyn Material>,
    ) -> Self {
        let (front_face, normal) = Self::face_normal(r, outward_normal);
        HitRecord {
            p,
            normal,
            mat_ptr,
            t,
            front_face,
        }
    }

    pub fn face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        (front_face, normal)
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
