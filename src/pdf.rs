use crate::{
    helper::random_f64,
    hittable::Hittable,
    onb::ONB,
    vec3::{Point3, Vec3},
};

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

pub struct HittablePDF<'a> {
    objects: &'a dyn Hittable,
    origin: Point3,
}

impl<'a> PDF for HittablePDF<'a> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

impl<'a> HittablePDF<'a> {
    pub fn new(objects: &'a dyn Hittable, &origin: &Point3) -> Self {
        Self { objects, origin }
    }
}

pub struct MixturePDF<'a> {
    p: [&'a dyn PDF; 2],
}

impl<'a> PDF for MixturePDF<'a> {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_f64() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}

impl<'a> MixturePDF<'a> {
    pub fn new(p0: &'a dyn PDF, p1: &'a dyn PDF) -> Self {
        Self { p: [p0, p1] }
    }
}
