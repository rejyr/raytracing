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

#[macro_export]
macro_rules! translate {
    ($object:expr, $offset:expr) => {
        std::sync::Arc::new($crate::hittable::Translate::new($object, &($offset)))
    };
}

#[derive(Clone)]
pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Transform the ray from world space to object space.
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Point3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        // Determine whether an intersection exists in object space (and if so, where).
        let Some(mut hr) = self.object.hit(&rotated_r, ray_t) else {
            return None;
        };

        // Transform the intersection from object space back to world space.
        hr.p = Point3::new(
            (self.cos_theta * hr.p.x()) + (self.sin_theta * hr.p.z()),
            hr.p.y(),
            (-self.sin_theta * hr.p.x()) + (self.cos_theta * hr.p.z()),
        );
        hr.normal = Point3::new(
            (self.cos_theta * hr.normal.x()) + (self.sin_theta * hr.normal.z()),
            hr.normal.y(),
            (-self.sin_theta * hr.normal.x()) + (self.cos_theta * hr.normal.z()),
        );

        Some(hr)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let x = i * bbox.x.max + (1.0 - i) * bbox.x.min;
                    let y = j * bbox.y.max + (1.0 - j) * bbox.y.min;
                    let z = k * bbox.z.max + (1.0 - k) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::from_points(&min, &max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

#[macro_export]
macro_rules! rotate_y {
    ($object:expr, $angle:expr) => {
        std::sync::Arc::new($crate::hittable::RotateY::new($object, $angle as f64))
    };
}
