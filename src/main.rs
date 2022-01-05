use std::io::Write;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", 255);

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    for j in (0..IMAGE_HEIGHT).rev() {
        write!(handle, "\rScanlines remaining: {}", j)?;
        handle.flush()?;
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let g = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    write!(handle, "\nDone.\n")?;
    Ok(())
}
