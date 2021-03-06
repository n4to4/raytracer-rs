use super::*;
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat_ptr: Arc<dyn Material + Send + Sync>) -> Self {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let hit_p = r.at(root);
        let outward_normal = (hit_p - self.center) / self.radius;
        Some(HitRecord::new_with(
            r,
            hit_p,
            root,
            outward_normal,
            Arc::clone(&self.mat_ptr),
        ))
    }
}
