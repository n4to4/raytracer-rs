use super::*;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Self {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_direction = {
            let v = rec.normal + Vec3::random_unit_vector();

            // Catch degenerate scatter direction
            if v.near_zero() {
                rec.normal
            } else {
                v
            }
        };

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
