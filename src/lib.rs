#![allow(clippy::new_without_default)]

pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use hittable::{HitRecord, Hittable};
pub use ray::Ray;
pub use vec3::Vec3;
