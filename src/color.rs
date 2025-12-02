use std::io::Write;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

const INTENSITY: Interval = Interval::new(0.000, 0.999);

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    0.0
}

pub fn write_color(out: &mut impl Write, pixel_color: &Color) -> Result<(), std::io::Error> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Apply a linear to gamma tranform for gamma 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (256.0 * INTENSITY.clamp(r)) as i32;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as i32;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as i32;

    // Write out the pixel color components.
    writeln!(out, "{rbyte} {gbyte} {bbyte}")
}
