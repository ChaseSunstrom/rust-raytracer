use crate::vec3::*;

pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Ray {
    pub fn default() -> Ray {
        Ray {
            orig: Point3::default(),
            dir: Vec3::default()
        }
    }

    pub fn new(orig: Point3, dir: Vec3) -> Ray {
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

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}