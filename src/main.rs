use raytracer::*;
use std::io::Write;
use std::rc::Rc;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    // If we've exeeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // World
    #[allow(non_snake_case)]
    let R = (std::f64::consts::PI / 4.0).cos();
    let mut world = HittableList::new();

    let mut world_add = |p: (f64, f64, f64), r: f64, m: Rc<dyn Material>| {
        world.add(Box::new(Sphere::new(Vec3::new(p.0, p.1, p.2), r, m)));
    };

    //let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    //let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0)));

    world_add((-R, 0.0, -1.0), R, material_left);
    world_add((R, 0.0, -1.0), R, material_right);

    //world_add((0.0, -100.5, -1.0), 100.0, material_ground);
    //world_add((0.0, 0.0, -1.0), 0.5, material_center);
    //world_add((-1.0, 0.0, -1.0), 0.5, material_left.clone());
    //world_add((-1.0, 0.0, -1.0), -0.4, material_left);
    //world_add((1.0, 0.0, -1.0), 0.5, material_right);

    // Camera
    let camera = Camera::new(90.0, ASPECT_RATIO);

    // Stdout/err
    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    // Render
    writeln!(stdout, "P3")?;
    writeln!(stdout, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(stdout, "{}", 255)?;

    for j in (0..IMAGE_HEIGHT).rev() {
        write!(stderr, "\rScanlines remaining: {}", j)?;
        stderr.flush()?;
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            writeln!(stdout, "{}", pixel_color.to_color(SAMPLES_PER_PIXEL))?;
        }
    }
    write!(stderr, "\nDone.\n")?;
    Ok(())
}
