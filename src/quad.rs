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
    w: Vec3,
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

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        Self::is_interior(alpha, beta)
            .map(|(u, v)| HitRecord::new(t, r, &self.normal, u, v, self.mat.clone()))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl Quad {
    pub fn new(q: &Point3, u: &Vec3, v: &Vec3, mat: Arc<dyn Material>) -> Self {
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);
        let w = n / n.dot(&n);

        Self {
            q: *q,
            u: *u,
            v: *v,
            w,
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

    /// Given the hit point in plane coordinates, return None if it is outside primitive, otherwise
    /// return Some<(u, v)>
    fn is_interior(a: f64, b: f64) -> Option<(f64, f64)> {
        let unit_interval = Interval::new(0.0, 1.0);

        (unit_interval.contains(a) && unit_interval.contains(b)).then_some((a, b))
    }
}

#[macro_export]
macro_rules! quad {
    ($q:expr, $u:expr, $v:expr, $mat:expr) => {
        std::sync::Arc::new($crate::quad::Quad::new(&($q), &($u), &($v), $mat.clone()))
    };
}
