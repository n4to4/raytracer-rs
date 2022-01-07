#![allow(clippy::new_without_default)]

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_f64() -> f64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}
