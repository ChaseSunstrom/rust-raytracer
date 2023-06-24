use crate::vec3::*;

pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            orig: Point3::new(),
            dir: Vec3::new()
        }
    }

    pub fn new_with_values(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            orig,
            dir
        }
    }

    pub fn get_origin(&self) -> Point3 {
        self.orig
    }

    pub fn get_direction(&self) -> Vec3 {
        self.dir
    }
}