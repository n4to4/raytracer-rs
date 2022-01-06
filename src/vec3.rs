use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(u: Self, v: Self) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    pub fn unit_vector(v: Self) -> Self {
        let len = v.length();
        v / len
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self::Output {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self::Output {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// color

type Color = Vec3;

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            255.999 * self.x,
            255.999 * self.y,
            255.999 * self.z
        )
    }
}
