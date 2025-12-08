use std::sync::Arc;

use crate::{
    color::Color,
    helper::random_f64,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::{Isotropic, Material},
    texture::{SolidColor, Texture},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        let Some(mut rec1) = self.boundary.hit(r, Interval::UNIVERSE) else {
            return None;
        };
        let Some(mut rec2) = self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f64::INFINITY))
        else {
            return None;
        };

        rec1.t = rec1.t.max(ray_t.min);
        rec2.t = rec2.t.min(ray_t.max);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_f64().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        let normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        let front_face = true; // also arbitrary
        let mat = self.phase_function.clone();

        Some(HitRecord {
            p,
            normal,
            mat,
            t,
            u: Default::default(),
            v: Default::default(),
            front_face,
        })
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        self.boundary.bounding_box()
    }
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }

    pub fn new_from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: &Color) -> Self {
        Self::new(boundary, density, Arc::new(SolidColor::new(albedo)))
    }
}

#[macro_export]
macro_rules! constant_medium {
    ($boundary:expr, $density:expr, $tex:expr) => {
        std::sync::Arc::new($crate::constant_medium::ConstantMedium::new(
            $boundary,
            $density as f64,
            $tex.clone(),
        ))
    };
    ($boundary:expr, $density:expr, color: $color:expr) => {
        std::sync::Arc::new($crate::constant_medium::ConstantMedium::new_from_color(
            $boundary,
            $density as f64,
            &($color),
        ))
    };
}
