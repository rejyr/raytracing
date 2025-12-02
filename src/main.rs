use std::error::Error;
use std::io::{BufWriter, Write, stdout};

use raytracing::color::{Color, write_color};
use raytracing::ray::Ray;
use raytracing::vec3::{Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - *r.origin();
    let a = r.direction().length_squared();
    let h = r.direction().dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = r.at(t) - Vec3::new(0.0, 0.0, -1.0);
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    };

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = {
        let image_height = image_width / aspect_ratio as i32;
        if image_height < 1 { 1 } else { image_height }
    };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height) as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut out = BufWriter::new(stdout());

    writeln!(out, "P3\n{image_width} {image_height}\n255")?;

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel100_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut out, &pixel_color)?;
        }
    }
    eprintln!("\rDone.");
    Ok(())
}
