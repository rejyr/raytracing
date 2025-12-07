use std::array;

use crate::{
    helper::{random_f64, random_i32_in_range},
    vec3::Point3,
};

#[derive(Debug, Clone)]
pub struct Perlin {
    randfloat: [f64; Self::POINT_COUNT],
    perm_x: [usize; Self::POINT_COUNT],
    perm_y: [usize; Self::POINT_COUNT],
    perm_z: [usize; Self::POINT_COUNT],
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        Self {
            randfloat: array::from_fn(|_| random_f64()),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;

        self.randfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }

    fn perlin_generate_perm() -> [usize; Self::POINT_COUNT] {
        let mut p = array::from_fn(|i| i);
        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut [usize]) {
        for i in (1..p.len()).rev() {
            let target = random_i32_in_range(0, i as i32) as usize;
            p.swap(target, i);
        }
    }
}
