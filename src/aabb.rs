use crate::{interval::Interval, ray::Ray, vec3::Point3};

#[derive(Debug, Clone, Copy, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub const EMPTY: Self = Self::new(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY);
    pub const UNIVERSE: Self =
        Self::new(Interval::UNIVERSE, Interval::UNIVERSE, Interval::UNIVERSE);

    pub const fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: &Point3, b: &Point3) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.
        let x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };
        let y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };
        let z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };
        Self::new(x, y, z)
    }

    pub fn from_aabbs(box0: &AABB, box1: &AABB) -> AABB {
        let x = Interval::from_intervals(&box0.x, &box1.x);
        let y = Interval::from_intervals(&box0.y, &box1.y);
        let z = Interval::from_intervals(&box0.z, &box1.z);
        Self::new(x, y, z)
    }

    pub const fn axis_interval(&self, n: usize) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let mut t0 = (ax.min - ray_orig[axis]) * adinv;
            let mut t1 = (ax.max - ray_orig[axis]) * adinv;
            if t0 > t1 {
                (t0, t1) = (t1, t0);
            }

            ray_t.min = ray_t.min.max(t0);
            ray_t.max = ray_t.max.min(t1);

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    /// Returns the index of the longest_axis of the bounding box.
    pub const fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }
}
