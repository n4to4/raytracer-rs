use rayon::prelude::*;
use raytracer::*;
use std::io::Write;
use std::sync::Arc;

// Image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: usize = 1200;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: i32 = 500;
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let mut world_add =
        |p: (f64, f64, f64), r: f64, m: Arc<dyn Material + Send + Sync + 'static>| {
            world.add(Arc::new(Sphere::new(Vec3::new(p.0, p.1, p.2), r, m)));
        };

    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world_add((0.0, -1000.0, 0.0), 1000.0, ground_material);

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = random_f64();
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world_add((center.x, center.y, center.z), 0.2, sphere_material);
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_range_f64(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world_add((center.x, center.y, center.z), 0.2, sphere_material);
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric { ir: 1.5 });
                    world_add((center.x, center.y, center.z), 0.2, sphere_material);
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric { ir: 1.5 });
    world_add((0.0, 1.0, 0.0), 1.0, material1);

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world_add((-4.0, 1.0, 0.0), 1.0, material2);

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world_add((4.0, 1.0, 0.0), 1.0, material3);

    world
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // World
    let world = random_scene();

    // Camera
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    // Stdout/err
    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    // Render
    writeln!(stdout, "P3")?;
    writeln!(stdout, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(stdout, "{}", 255)?;

    struct Line {
        key: (usize, usize),
        color: Color,
    }

    let mut lines: Vec<Line> = (0..IMAGE_HEIGHT)
        .rev()
        .flat_map(|j| {
            write!(stderr, "\rScanlines remaining: {}", j).unwrap();
            (0..IMAGE_WIDTH)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (i as f64 + random_f64()) / (IMAGE_WIDTH - 1) as f64;
                        let v = (j as f64 + random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                        let r = camera.ray(u, v);
                        pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
                    }
                    let color = pixel_color.to_color(SAMPLES_PER_PIXEL);
                    Line {
                        key: (IMAGE_HEIGHT - j + 1, i),
                        color,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    lines.par_sort_by(|a, b| a.key.cmp(&b.key));
    for line in &lines {
        writeln!(stdout, "{}", line.color)?;
    }

    write!(stderr, "\nDone.\n")?;
    Ok(())
}
