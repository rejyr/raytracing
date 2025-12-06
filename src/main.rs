use raytracing::{color, material, sphere, vec3};
use std::error::Error;
use std::io::{BufWriter, stdout};

use raytracing::camera::{Camera, CameraConfig};
use raytracing::color::Color;
use raytracing::helper::{random_f64, random_f64_in_range};
use raytracing::hittable_list::HittableList;
use raytracing::point3;

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::default();

    let ground_material = material!(Lambertian(color!(0.5, 0.5, 0.5)));
    world.add(sphere!(point3!(0, -1000, 0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = point3!(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - point3!(4, 0.2, 0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let center2 = center + vec3!(0, random_f64_in_range(0.0, 0.5), 0);
                    let sphere_material = material!(Lambertian(albedo));
                    world.add(sphere!(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    let sphere_material = material!(Metal(albedo, fuzz));
                    world.add(sphere!(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = material!(Dielectric(1.5));
                    world.add(sphere!(center, 0.2, sphere_material));
                };
            }
        }
    }

    let material1 = material!(Dielectric(1.5));
    world.add(sphere!(point3!(0, 1, 0), 1.0, material1));

    let material2 = material!(Lambertian(color!(0.4, 0.2, 0.1)));
    world.add(sphere!(point3!(-4, 1, 0), 1.0, material2));

    let material3 = material!(Metal(color!(0.7, 0.6, 0.5), 0));
    world.add(sphere!(point3!(4, 1, 0), 1.0, material3));

    let cc = CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: point3!(13, 2, 3),
        lookat: point3!(0, 0, 0),
        vup: vec3!(0, 1, 0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
    };
    let cam = Camera::from_config(cc);

    let mut out = BufWriter::new(stdout());
    cam.render(&mut out, &world)?;

    Ok(())
}
