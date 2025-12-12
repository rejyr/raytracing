use raytracing::quad::make_box;
use raytracing::{color, material, quad, rotate_y, sphere, translate, vec3};
use std::error::Error;
use std::io::{BufWriter, stdout};

use raytracing::camera::{Camera, CameraConfig};
use raytracing::hittable_list::HittableList;
use raytracing::point3;

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let red = material!(Lambertian(color: color!(0.65, 0.05, 0.05)));
    let white = material!(Lambertian(color: color!(0.73,0.73,0.73)));
    let green = material!(Lambertian(color: color!(0.12,0.45,0.15)));
    let light = material!(DiffuseLight(color: color!(15,15,15)));

    world.add(quad!(
        point3!(550, 0, 0),
        vec3!(0, 555, 0),
        vec3!(0, 0, 555),
        green
    ));
    world.add(quad!(
        point3!(0, 0, 0),
        vec3!(0, 555, 0),
        vec3!(0, 0, 555),
        red
    ));
    world.add(quad!(
        point3!(343, 554, 332),
        vec3!(-130, 0, 0),
        vec3!(0, 0, -105),
        light
    ));
    world.add(quad!(
        point3!(0, 0, 0),
        vec3!(555, 0, 0),
        vec3!(0, 0, 555),
        white
    ));
    world.add(quad!(
        point3!(555, 555, 555),
        vec3!(-555, 0, 0),
        vec3!(0, 0, -555),
        white
    ));
    world.add(quad!(
        point3!(0, 0, 555),
        vec3!(555, 0, 0),
        vec3!(0, 555, 0),
        white
    ));

    // Box
    let box1 = make_box(&point3!(0, 0, 0), &point3!(165, 330, 165), white);
    let box1 = rotate_y!(box1, 15);
    let box1 = translate!(box1, vec3!(265, 0, 295));
    world.add(box1);

    // Glass Sphere
    let glass = material!(Dielectric(1.5));
    world.add(sphere!(point3!(190, 90, 190), 90, glass));

    // Light sources
    let lights = quad!(
        point3!(343, 554, 332),
        vec3!(-130, 0, 0),
        vec3!(0, 0, -105),
        light
    );

    let cc = CameraConfig {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 1000,
        max_depth: 50,
        vfov: 40.0,
        lookfrom: point3!(278, 278, -800),
        lookat: point3!(278, 278, 0),
        vup: vec3!(0, 1, 0),
        defocus_angle: 0.0,
        background: color!(0, 0, 0),
        ..Default::default()
    };
    let cam = Camera::from_config(cc);

    let mut out = BufWriter::new(stdout());
    cam.render(&mut out, &world, &*lights)?;

    Ok(())
}
