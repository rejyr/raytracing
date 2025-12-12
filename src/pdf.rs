use crate::{onb::ONB, vec3::Vec3};

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

#[derive(Debug, Clone, Copy)]
pub struct SpherePDF;

impl PDF for SpherePDF {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

#[derive(Debug, Clone)]
pub struct CosinePDF {
    uvw: ONB,
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = direction.unit_vector().dot(self.uvw.w());
        (cosine_theta / std::f64::consts::PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(&Vec3::random_cosine_direction())
    }
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: ONB::new(w) }
    }
}
