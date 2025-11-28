use std::error::Error;
use std::io::{BufWriter, Write, stdout};

use raytracing::color::{Color, write_color};

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() -> Result<(), Box<dyn Error>> {
    let mut out = BufWriter::new(stdout());

    writeln!(out, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255")?;

    for j in 0..IMAGE_HEIGHT {
        eprintln!("\rScanlines remaining: {} ", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            );
            write_color(&mut out, &pixel_color)?;
        }
    }
    eprintln!("\rDone.");
    Ok(())
}
