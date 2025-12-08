use std::sync::Arc;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, r: &Ray, normal: &Vec3, u: f64, v: f64, mat: Arc<dyn Material>) -> Self {
        let p = r.at(t);
        let front_face = r.direction().dot(normal) < 0.0;
        let normal = if front_face { *normal } else { -*normal };

        Self {
            p,
            normal,
            mat,
            t,
            u,
            v,
            front_face,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}

#[derive(Clone)]
pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_r = Ray::new_with_time(*r.origin() - self.offset, *r.direction(), r.time());

        // Determine whether an intersection exists along the offset ray (and if so, where)
        self.object.hit(&offset_r, ray_t).map(|mut hr| {
            hr.p += self.offset;
            hr
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: &Vec3) -> Self {
        Self {
            bbox: object.bounding_box() + *offset,
            object,
            offset: *offset,
        }
    }
}
