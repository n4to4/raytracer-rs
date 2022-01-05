use raytracer::*;
use std::io::Write;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    write!(stdout, "P3")?;
    write!(stdout, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    write!(stdout, "{}", 255)?;

    for j in (0..IMAGE_HEIGHT).rev() {
        write!(stderr, "\rScanlines remaining: {}", j)?;
        stderr.flush()?;
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let g = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;
            write!(stdout, "{}", Vec3::new(r, g, b))?;
        }
    }
    write!(stderr, "\nDone.\n")?;
    Ok(())
}
