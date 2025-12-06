use std::io::Result;
use std::io::Write;
use std::sync::atomic::AtomicUsize;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::color::write_color;
use crate::helper::random_f64;
use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct CameraConfig {
    /// Ratio of image width over height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel cout
    pub image_width: usize,
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
    image_height: usize,
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
            let image_height = (cc.image_width as f64 / cc.aspect_ratio) as usize;
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
    fn get_ray(&self, i: usize, j: usize) -> Ray {
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

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some(sr) = rec.mat.scatter(r, &rec) {
                return sr.attenuation * Self::ray_color(&sr.scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    pub fn render(&self, out: &mut impl Write, world: &impl Hittable) -> Result<()> {
        let w = self.config.image_width;
        let h = self.image_height;
        // i = n % w, j = n / w
        // n = i + j*w
        let mut buf = vec![Color::default(); w * h];

        let lines_done = AtomicUsize::new(0);

        buf.par_iter_mut().enumerate().for_each(|(n, c)| {
            let i = n % w;
            let j = n / w;
            *c = self.render_pixel(i, j, world);

            if n % w == 0 {
                let lines_done = lines_done.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                if lines_done.is_multiple_of(10) {
                    eprintln!("Scanlines left: {}", h - lines_done);
                }
            }
        });

        writeln!(out, "P3\n{} {}\n255", w, h)?;
        for c in buf {
            write_color(out, &c)?;
        }

        eprintln!("\rDone.");

        Ok(())
    }

    fn render_pixel(&self, i: usize, j: usize, world: &impl Hittable) -> Color {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..self.config.samples_per_pixel {
            let r = self.get_ray(i, j);
            pixel_color += Self::ray_color(&r, self.config.max_depth, world);
        }
        self.pixel_samples_scale * pixel_color
    }
}
