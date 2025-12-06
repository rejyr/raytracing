use crate::vec3::{Point3, Vec3};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

// TODO: upgrade functions to `const`
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn origin_mut(&mut self) -> &mut Point3 {
        &mut self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub fn direction_mut(&mut self) -> &mut Vec3 {
        &mut self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
