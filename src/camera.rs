use std::io::Result;
use std::io::Write;

use crate::color::write_color;
use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct CameraConfig {
    /// Ratio of image width over height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel cout
    pub image_width: i32,
    /// Count of random samples for each pixel
    pub samples_per_pixel: i32,
    /// Maximum number of ray bounces into scene
    pub max_depth: i32,

    /// Vertical view angle (field of view)
    pub vfov: f64,
    /// Point camera is looking from
    pub lookfrom: Point3,
    /// Point camera is looking at
    pub lookat: Point3,
    /// Camera-relative "up" direction
    pub vup: Vec3,

    /// Variation angle of rays through each pixel
    pub defocus_angle: f64,
    /// Distance from the camera lookfrom point to plane of perfect focus
    pub focus_dist: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,

            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Camera {
    config: CameraConfig,

    /// Rendered image height
    image_height: i32,
    /// Color scale factor to for a sum of pixels
    pixel_samples_scale: f64,
    /// Camera center
    center: Point3,
    /// Location of pixel 0, 0
    pixel100_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    /// Defocus disk vertical radius
    defocus_disk_v: Vec3,
}

impl From<CameraConfig> for Camera {
    fn from(cc: CameraConfig) -> Self {
        let image_height = {
            let image_height = (cc.image_width as f64 / cc.aspect_ratio) as i32;
            if image_height < 1 { 1 } else { image_height }
        };

        let pixel_samples_scale = 1.0 / cc.samples_per_pixel as f64;

        let center = cc.lookfrom;

        // Determine viewport dimensions.
        let theta = cc.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * cc.focus_dist;
        let viewport_width = viewport_height * (cc.image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (cc.lookfrom - cc.lookat).unit_vector();
        let u = cc.vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Caclulate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / cc.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - (cc.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = cc.focus_dist * (cc.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            config: cc,
            image_height,
            pixel_samples_scale,
            center,
            pixel100_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

impl Camera {
    pub fn from_config(cc: CameraConfig) -> Self {
        cc.into()
    }

    /// Construct a camera ray originating from the origin and directed at randomly sampled point
    /// around the pixel location i, j.
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel100_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.config.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default_with_default_lambertian();

        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    fn sample_square() -> Vec3 {
        Vec3::new(fastrand::f64() - 0.5, fastrand::f64() - 0.5, 0.0)
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    // TODO: refactor to use image library
    // TODO: add parallelism
    pub fn render(&self, out: &mut impl Write, world: &dyn Hittable) -> Result<()> {
        writeln!(
            out,
            "P3\n{} {}\n255",
            self.config.image_width, self.image_height
        )?;

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.config.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.config.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.config.max_depth, world);
                }
                write_color(out, &(self.pixel_samples_scale * pixel_color))?;
            }
        }
        eprintln!("\rDone.");

        Ok(())
    }
}
