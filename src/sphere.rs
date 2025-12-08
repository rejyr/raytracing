use std::{f64::consts::PI, sync::Arc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_t.min || ray_t.max <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.min || ray_t.max <= root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let normal = (p - current_center) / self.radius;
        let (u, v) = Self::get_sphere_uv(&normal);

        Some(HitRecord::new(t, r, &normal, u, v, self.mat.clone()))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AABB::from_points(&(static_center - rvec), &(static_center + rvec));
        Self {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius,
            mat,
            bbox,
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Arc<dyn Material>,
    ) -> Self {
        let center = Ray::new(center1, center2 - center1);

        let rvec = Vec3::new(radius, radius, radius);
        let bbox1 = AABB::from_points(&(center.at(0.0) - rvec), &(center.at(0.0) + rvec));
        let bbox2 = AABB::from_points(&(center.at(1.0) - rvec), &(center.at(1.0) + rvec));
        let bbox = AABB::from_aabbs(&bbox1, &bbox2);

        Self {
            center,
            radius: radius.max(0.0),
            mat,
            bbox,
        }
    }

    /// p: a given point on the sphere of radius one, centered at the origin
    /// returns (u, v)
    /// u: returned value [0,1] of angle around the Y axis from X=-1
    /// v: returned value [0,1] of angle from Y=-1 to Y=-1
    /// <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    /// <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    /// <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

#[macro_export]
macro_rules! sphere {
    ($center:expr, $radius:expr, $mat:expr) => {
        std::sync::Arc::new($crate::sphere::Sphere::new(
            $center,
            $radius as f64,
            $mat.clone(),
        ))
    };
    ($center1:expr, $center2:expr, $radius:expr, $mat:expr) => {
        std::sync::Arc::new($crate::sphere::Sphere::new_moving(
            $center1,
            $center2,
            $radius,
            $mat.clone(),
        ))
    };
}
