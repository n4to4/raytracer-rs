use raytracer::ray::Ray;
use raytracer::vec3::Vec3;
use std::io::Write;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

// Camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = Vec3::dot(r.direction, r.direction);
    let half_b = Vec3::dot(oc, r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    writeln!(stdout, "P3")?;
    writeln!(stdout, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(stdout, "{}", 255)?;

    for j in (0..IMAGE_HEIGHT).rev() {
        write!(stderr, "\rScanlines remaining: {}", j)?;
        stderr.flush()?;
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            writeln!(stdout, "{}", pixel_color)?;
        }
    }
    write!(stderr, "\nDone.\n")?;
    Ok(())
}
