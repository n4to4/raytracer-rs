use super::*;

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(color: Vec3) -> Self {
        Metal { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::unit_vector(r_in.direction).reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
