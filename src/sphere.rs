use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}
