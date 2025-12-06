use std::rc::Rc;

use crate::{
    interval::Interval,
    material::Material,
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

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
