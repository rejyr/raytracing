use raytracing::bvh::BVHNode;
use raytracing::{color, material, quad, sphere, texture, vec3};
use std::error::Error;
use std::io::{BufWriter, stdout};

use raytracing::camera::{Camera, CameraConfig};
use raytracing::color::Color;
use raytracing::helper::{random_f64, random_f64_in_range};
use raytracing::hittable_list::HittableList;
use raytracing::point3;

fn bouncing_spheres() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::default();

    let checker =
        texture!(CheckerTexture(0.32, color: color!(0.2,0.3, 0.1), color: color!(0.9, 0.9, 0.90)));
    world.add(sphere!(
        point3!(0, -1000, 0),
        1000.0,
        material!(Lambertian(checker))
    ));

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
                    let sphere_material = material!(Lambertian(color: albedo));
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

    let material2 = material!(Lambertian(color: color!(0.4, 0.2, 0.1)));
    world.add(sphere!(point3!(-4, 1, 0), 1.0, material2));

    let material3 = material!(Metal(color!(0.7, 0.6, 0.5), 0));
    world.add(sphere!(point3!(4, 1, 0), 1.0, material3));

    let bvh = BVHNode::from_list(world);

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
    cam.render(&mut out, &bvh)?;

    Ok(())
}

fn checkered_spheres() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::default();

    let checker =
        texture!(CheckerTexture(0.32, color: color!(0.2,0.3, 0.1), color: color!(0.9, 0.9, 0.90)));

    world.add(sphere!(
        point3!(0, -10, 0),
        10,
        material!(Lambertian(checker.clone()))
    ));
    world.add(sphere!(
        point3!(0, 10, 0),
        10,
        material!(Lambertian(checker))
    ));

    let cc = CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: point3!(13, 2, 3),
        lookat: point3!(0, 0, 0),
        vup: vec3!(0, 1, 0),
        defocus_angle: 0.0,
        ..Default::default()
    };
    let cam = Camera::from_config(cc);

    let mut out = BufWriter::new(stdout());
    cam.render(&mut out, &world)?;

    Ok(())
}

fn earth() -> Result<(), Box<dyn Error>> {
    let earth_texture = texture!(ImageTexture("textures/earthmap.jpg"));
    let earth_surface = material!(Lambertian(earth_texture));
    let globe = sphere!(point3!(0, 0, 0), 2, earth_surface);

    let cc = CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: point3!(0, 0, 12),
        lookat: point3!(0, 0, 0),
        vup: vec3!(0, 1, 0),
        defocus_angle: 0.0,
        ..Default::default()
    };
    let cam = Camera::from_config(cc);

    let mut out = BufWriter::new(stdout());
    cam.render(&mut out, globe.as_ref())?;

    Ok(())
}

fn perlin_spheres() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let pertext = texture!(NoiseTexture(4));
    world.add(sphere!(
        point3!(0, -1000, 0),
        1000,
        material!(Lambertian(pertext.clone()))
    ));
    world.add(sphere!(point3!(0, 2, 0), 2, material!(Lambertian(pertext))));

    let cc = CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: point3!(13, 2, 3),
        lookat: point3!(0, 0, 0),
        vup: vec3!(0, 1, 0),
        defocus_angle: 0.0,
        ..Default::default()
    };
    let cam = Camera::from_config(cc);

    let mut out = BufWriter::new(stdout());
    cam.render(&mut out, &world)?;

    Ok(())
}

fn quads() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let left_red = material!(Lambertian(color: color!(1, 0.2, 0.2)));
    let back_green = material!(Lambertian(color: color!(0.2, 1, 0.2)));
    let right_blue = material!(Lambertian(color: color!(0.2, 0.2, 1)));
    let upper_orange = material!(Lambertian(color: color!(1, 0.5, 0)));
    let lower_teal = material!(Lambertian(color: color!(0.2, 0.8, 0.8)));

    world.add(quad!(
        point3!(-3, -2, 5),
        vec3!(0, 0, -4),
        vec3!(0, 4, 0),
        left_red
    ));
    world.add(quad!(
        point3!(-2, -2, 0),
        vec3!(4, 0, 0),
        vec3!(0, 4, 0),
        back_green
    ));
    world.add(quad!(
        point3!(3, -2, 1),
        vec3!(0, 0, 4),
        vec3!(0, 4, 0),
        right_blue
    ));
    world.add(quad!(
        point3!(-2, 3, 1),
        vec3!(4, 0, 0),
        vec3!(0, 0, 4),
        upper_orange
    ));
    world.add(quad!(
        point3!(-2, -3, 5),
        vec3!(4, 0, 0),
        vec3!(0, 0, -4),
        lower_teal
    ));

    let cc = CameraConfig {
        aspect_ratio: 1.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 80.0,
        lookfrom: point3!(0, 0, 9),
        lookat: point3!(0, 0, 0),
        vup: vec3!(0, 1, 0),
        defocus_angle: 0.0,
        ..Default::default()
    };
    let cam = Camera::from_config(cc);

    let mut out = BufWriter::new(stdout());
    cam.render(&mut out, &world)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    match 5 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        _ => unimplemented!(),
    }
}
