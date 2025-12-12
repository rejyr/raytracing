use std::sync::Arc;

use crate::{
    aabb::AABB,
    helper::random_i32_in_range,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(hit_rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_rec.t;
                temp_rec = Some(hit_rec);
            }
        }

        temp_rec
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        self.objects
            .iter()
            .map(|object| weight * object.pdf_value(origin, direction))
            .sum()
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let int_len = self.objects.len() as i32;
        let random_i = random_i32_in_range(0, int_len - 1) as usize;
        self.objects[random_i].random(origin)
    }
}

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::from_aabbs(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
