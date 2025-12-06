use std::rc::Rc;

use crate::{
    interval::Interval,
    material::{Lambertian, Material},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default_with_material(mat: Rc<dyn Material>) -> Self {
        Self {
            p: Default::default(),
            normal: Default::default(),
            mat,
            t: Default::default(),
            front_face: Default::default(),
        }
    }

    pub fn default_with_default_lambertian() -> Self {
        Self::default_with_material(Rc::new(Lambertian::default()))
    }

    /// Sets the hit record normal vector.
    /// NOTE: the parameter `outward_normal` is assumed to have unit length
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    // TODO: refactor to return `Option<HitRecord>`
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
