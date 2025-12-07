use std::sync::Arc;

use crate::{color::Color, vec3::Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[macro_export]
macro_rules! texture {
    (SolidColor( $albedo:expr )) => {
        std::sync::Arc::new($crate::texture::SolidColor::new(&($albedo)))
    };
    (CheckerTexture( $scale:expr, $even:expr, $odd:expr )) => {
        std::sync::Arc::new($crate::texture::CheckerTexture::new($scale, $even, $odd))
    };
    (CheckerTexture( $scale:expr, color: $even_color:expr, color: $odd_color:expr )) => {
        std::sync::Arc::new($crate::texture::CheckerTexture::new_from_colors(
            $scale,
            &($even_color),
            &($odd_color),
        ))
    };
}

#[derive(Debug, Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

impl SolidColor {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self::new(&Color::new(red, green, blue))
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_from_colors(scale: f64, even: &Color, odd: &Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(even)),
            odd: Arc::new(SolidColor::new(odd)),
        }
    }
}
