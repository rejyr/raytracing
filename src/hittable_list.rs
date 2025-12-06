use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
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
}

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
