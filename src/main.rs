use std::error::Error;
use std::io::{BufWriter, stdout};
use std::rc::Rc;

use raytracing::camera::Camera;
use raytracing::color::Color;
use raytracing::helper::random_f64_in_range;
use raytracing::hittable_list::HittableList;
use raytracing::material::{Dielectric, Lambertian, Material, Metal};
use raytracing::sphere::Sphere;
use raytracing::vec3::{Point3, Vec3};

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::default();

    let ground_material = Rc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f64();
            let center = Point3::new(
                a as f64 + 0.9 * fastrand::f64(),
                0.2,
                b as f64 + 0.9 * fastrand::f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Rc::new(Lambertian::new(&albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    Rc::new(Metal::new(&albedo, fuzz))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5))
                };
                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // TODO: reduce verbosity with macros?
    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Rc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    let mut out = BufWriter::new(stdout());
    cam.render(&mut out, &world)?;

    Ok(())
}
