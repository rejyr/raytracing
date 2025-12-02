use std::error::Error;
use std::io::{BufWriter, stdout};
use std::rc::Rc;

use raytracing::camera::Camera;
use raytracing::hittable_list::HittableList;
use raytracing::sphere::Sphere;
use raytracing::vec3::Point3;

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    let mut out = BufWriter::new(stdout());
    cam.render(&mut out, &world)?;

    Ok(())
}
