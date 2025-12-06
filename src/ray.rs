use crate::vec3::{Point3, Vec3};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f64,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self::new_with_time(origin, direction, 0.0)
    }

    pub const fn new_with_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            time,
        }
    }

    pub const fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub const fn origin_mut(&mut self) -> &mut Point3 {
        &mut self.orig
    }

    pub const fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub const fn direction_mut(&mut self) -> &mut Vec3 {
        &mut self.dir
    }

    pub const fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
