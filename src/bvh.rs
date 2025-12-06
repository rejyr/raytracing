use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
};

#[derive(Clone)]
pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);
        let t_max = match &hit_left {
            Some(rec) => rec.t,
            None => ray_t.max,
        };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, t_max));

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl From<HittableList> for BVHNode {
    fn from(mut list: HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }
}

impl BVHNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>], start: usize, end: usize) -> Self {
        // Build the bounding box of the span of the source objects.
        let mut bbox = AABB::EMPTY;
        for object in &objects[start..end] {
            bbox = AABB::from_aabbs(&bbox, &object.bounding_box());
        }
        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = end - start;

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match object_span {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => (objects[start].clone(), objects[start + 1].clone()),
            _ => {
                objects[start..end].sort_by(comparator);

                let mid = start + object_span / 2;
                (
                    Arc::new(Self::new(objects, start, mid)),
                    Arc::new(Self::new(objects, mid, end)),
                )
            }
        };

        Self { left, right, bbox }
    }

    pub fn from_list(list: HittableList) -> Self {
        list.into()
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .expect("BVHNode box_compare failed")
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}
