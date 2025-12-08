use raytracing::bvh::BVHNode;
use raytracing::quad::make_box;
use raytracing::vec3::Point3;
use raytracing::{
    color, constant_medium, material, quad, rotate_y, sphere, texture, translate, vec3,
};
use std::error::Error;
use std::io::{BufWriter, stdout};
use std::sync::Arc;

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
                }
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
        ..Default::default()
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
        material!(Lambertian(checker))
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

fn simple_light() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let pertext = texture!(NoiseTexture(4));
    world.add(sphere!(
        point3!(0, -1000, 0),
        1000,
        material!(Lambertian(pertext))
    ));
    world.add(sphere!(point3!(0, 2, 0), 2, material!(Lambertian(pertext))));

    let difflight = material!(DiffuseLight(color: color!(4, 4, 4)));
    world.add(sphere!(point3!(0, 7, 0), 2, difflight));
    world.add(quad!(
        point3!(3, 1, -2),
        vec3!(2, 0, 0),
        vec3!(0, 2, 0),
        difflight
    ));

    let cc = CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: point3!(26, 3, 6),
        lookat: point3!(0, 2, 0),
        vup: vec3!(0, 1, 0),
        defocus_angle: 0.0,
        background: color!(0, 0, 0),
        ..Default::default()
    };
    let cam = Camera::from_config(cc);

    let mut out = BufWriter::new(stdout());
    cam.render(&mut out, &world)?;

    Ok(())
}

fn cornell_box() -> Result<(), Box<dyn Error>> {
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

    let box1 = make_box(&point3!(0, 0, 0), &point3!(165, 330, 165), white.clone());
    let box1 = rotate_y!(box1, 15);
    let box1 = translate!(box1, vec3!(265, 0, 295));
    world.add(box1);

    let box2 = make_box(&point3!(0, 0, 0), &point3!(165, 165, 165), white);
    let box2 = rotate_y!(box2, -18);
    let box2 = translate!(box2, vec3!(130, 0, 65));
    world.add(box2);

    let cc = CameraConfig {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 200,
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
    cam.render(&mut out, &world)?;

    Ok(())
}

fn cornell_smoke() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let red = material!(Lambertian(color: color!(0.65, 0.05, 0.05)));
    let white = material!(Lambertian(color: color!(0.73,0.73,0.73)));
    let green = material!(Lambertian(color: color!(0.12,0.45,0.15)));
    let light = material!(DiffuseLight(color: color!(7,7,7)));

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
        point3!(113, 554, 127),
        vec3!(330, 0, 0),
        vec3!(0, 0, 305),
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

    let box1 = make_box(&point3!(0, 0, 0), &point3!(165, 330, 165), white.clone());
    let box1 = rotate_y!(box1, 15);
    let box1 = translate!(box1, vec3!(265, 0, 295));

    let box2 = make_box(&point3!(0, 0, 0), &point3!(165, 165, 165), white);
    let box2 = rotate_y!(box2, -18);
    let box2 = translate!(box2, vec3!(130, 0, 65));

    world.add(constant_medium!(box1, 0.01, color: color!(0,0,0)));
    world.add(constant_medium!(box2, 0.01, color: color!(1,1,1)));

    let cc = CameraConfig {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 200,
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
    cam.render(&mut out, &world)?;

    Ok(())
}

fn final_scene(
    image_width: usize,
    samples_per_pixel: i32,
    max_depth: i32,
) -> Result<(), Box<dyn Error>> {
    let mut boxes1 = HittableList::new();
    let ground = material!(Lambertian(color: color!(0.45,0.83,0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_in_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(make_box(
                &point3!(x0, y0, z0),
                &point3!(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();

    world.add(Arc::new(BVHNode::from_list(boxes1)));

    let light = material!(DiffuseLight(color: color!(7,7,7)));
    world.add(quad!(
        point3!(123, 554, 147),
        vec3!(300, 0, 0),
        vec3!(0, 0, 265),
        light
    ));

    let center1 = point3!(400, 400, 200);
    let center2 = center1 + vec3!(30, 0, 0);
    let sphere_material = material!(Lambertian(color: color!(0.7,0.3,0.1)));
    world.add(sphere!(center1, center2, 50, sphere_material));

    world.add(sphere!(
        point3!(260, 150, 45),
        50,
        material!(Dielectric(1.5))
    ));
    world.add(sphere!(
        point3!(0, 150, 145),
        50,
        material!(Metal(color!(0.8, 0.8, 0.9), 1))
    ));

    let boundary = sphere!(point3!(360, 150, 145), 70, material!(Dielectric(1.5)));
    world.add(boundary.clone());
    world.add(constant_medium!(boundary, 0.2, color: color!(0.2,0.4,0.9)));
    let boundary = sphere!(point3!(0, 0, 0), 5000, material!(Dielectric(1.5)));
    world.add(constant_medium!(boundary, 0.0001, color: color!(1,1,1)));

    let emat = material!(Lambertian(texture!(ImageTexture("textures/earthmap.jpg"))));
    world.add(sphere!(point3!(400, 200, 400), 100, emat));
    let pertext = texture!(NoiseTexture(0.2));
    world.add(sphere!(
        point3!(220, 280, 300),
        80,
        material!(Lambertian(pertext))
    ));

    let mut boxes2 = HittableList::new();
    let white = material!(Lambertian(color: color!(0.73,0.73,0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(sphere!(Point3::random_in_range(0.0, 165.0), 10, white));
    }

    world.add(translate!(
        rotate_y!(Arc::new(BVHNode::from_list(boxes2)), 15),
        vec3!(-100, 270, 395)
    ));

    let cc = CameraConfig {
        aspect_ratio: 1.0,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov: 40.0,
        lookfrom: point3!(478, 278, -600),
        lookat: point3!(278, 278, 0),
        vup: vec3!(0, 1, 0),
        defocus_angle: 0.0,
        background: color!(0, 0, 0),
        ..Default::default()
    };
    let cam = Camera::from_config(cc);

    let mut out = BufWriter::new(stdout());
    cam.render(&mut out, &world)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    match 0 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        _ => final_scene(400, 250, 4),
    }
}
