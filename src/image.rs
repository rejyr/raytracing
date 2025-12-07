use std::error::Error;

use image::{ImageReader, Rgb32FImage};

use crate::color::Color;

#[derive(Debug, Clone)]
pub struct RTWImage {
    image: Rgb32FImage,
}

impl RTWImage {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let image = ImageReader::open(path)?.decode()?.into_rgb32f();
        Ok(Self { image })
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> Color {
        let x = x.clamp(0, self.width());
        let y = y.clamp(0, self.height());

        let pixel = self.image.get_pixel(x, y);
        Color::new(pixel[0].into(), pixel[1].into(), pixel[2].into())
    }
}
