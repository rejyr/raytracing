use std::{error::Error, sync::Arc};

use crate::{color::Color, image::RTWImage, perlin::Perlin, vec3::Point3};

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
    (ImageTexture( $path:expr )) => {
        std::sync::Arc::new(
            $crate::texture::ImageTexture::new($path)
                .expect(&format!("Cannot open ImageTexture at path: {}", $path)),
        )
    };
    (NoiseTexture( $scale:expr )) => {
        std::sync::Arc::new($crate::texture::NoiseTexture::new($scale as f64))
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

#[derive(Debug, Clone)]
pub struct ImageTexture {
    image: RTWImage,
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = u * self.image.width() as f64;
        let j = v * self.image.height() as f64;

        self.image.pixel_data(i as u32, j as u32)
    }
}

impl ImageTexture {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let image = RTWImage::load(path)?;
        Ok(Self { image })
    }
}

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * *p)))
    }
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}
