use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    d: f64,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(r.direction());

        // No hit if ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return None if the hit point parameter if outside the ray interval.
        let t = (self.d - self.normal.dot(r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        Some(HitRecord::new(
            t,
            r,
            &self.normal,
            Default::default(), // TODO: calculate quad u and v
            Default::default(),
            self.mat.clone(),
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl Quad {
    pub fn new(q: &Point3, u: &Vec3, v: &Vec3, mat: Arc<dyn Material>) -> Self {
        let normal = u.cross(v).unit_vector();
        let d = normal.dot(q);

        Self {
            q: *q,
            u: *u,
            v: *v,
            normal,
            d,
            mat,
            bbox: Self::get_bounding_box(q, u, v),
        }
    }

    /// Compute the bounding box of all four vertices.
    fn get_bounding_box(q: &Point3, u: &Vec3, v: &Vec3) -> AABB {
        let bbox_diagonal1 = AABB::from_points(q, &(*q + *u + *v));
        let bbox_diagonal2 = AABB::from_points(&(*q + *u), &(*q + *v));
        AABB::from_aabbs(&bbox_diagonal1, &bbox_diagonal2)
    }
}
