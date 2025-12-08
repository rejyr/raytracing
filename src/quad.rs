use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::Hittable,
    material::Material,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Hittable for Quad {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        todo!()
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl Quad {
    pub fn new(q: &Point3, u: &Vec3, v: &Vec3, mat: Arc<dyn Material>) -> Self {
        Self {
            q: *q,
            u: *u,
            v: *v,
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
