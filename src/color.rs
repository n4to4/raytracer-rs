use super::*;

pub struct Color(Vec3);

impl Vec3 {
    pub fn to_color(mut self, samples_per_pixel: i32) -> Color {
        let r = &mut self.x;
        let g = &mut self.y;
        let b = &mut self.z;

        // Divide the color by the number of samples
        // and gamma-correct for gamma=2.0.
        let scale = 1.0 / samples_per_pixel as f64;
        *r = (scale * *r).sqrt();
        *g = (scale * *g).sqrt();
        *b = (scale * *b).sqrt();

        Color(self)
    }
}

// write_color
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write the translated [0,255] value of each color component.
        write!(
            f,
            "{} {} {}",
            256.0 * clamp(self.0.x, 0.0, 0.999),
            256.0 * clamp(self.0.y, 0.0, 0.999),
            256.0 * clamp(self.0.z, 0.0, 0.999),
        )
    }
}
