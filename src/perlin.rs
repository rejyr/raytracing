use std::array;

use crate::{
    helper::random_i32_in_range,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Perlin {
    randvec: [Vec3; Self::POINT_COUNT],
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
            randvec: array::from_fn(|_| Vec3::random_unit_vector()),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cij) in ci.iter_mut().enumerate() {
                for (dk, cijk) in cij.iter_mut().enumerate() {
                    let di = di as i32;
                    let dj = dj as i32;
                    let dk = dk as i32;
                    *cijk = self.randvec[self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize]]
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
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

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for (i, ci) in c.iter().enumerate() {
            for (j, cij) in ci.iter().enumerate() {
                for (k, cijk) in cij.iter().enumerate() {
                    let i = i as f64;
                    let f = j as f64;
                    let k = k as f64;
                    let weight_v = Vec3::new(u - i, v - f, w - k);
                    accum += (i * uu + (1.0 - i) * (1.0 - uu))
                        * (f * vv + (1.0 - f) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww))
                        * cijk.dot(&weight_v);
                }
            }
        }

        accum
    }
}
