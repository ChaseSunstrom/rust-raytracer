use std::rc::Rc;
use crate::vec3::{Point3, Vec3};
use crate::ray::*;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn default() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn new(objects: Rc<dyn Hittable>) -> HittableList {
        HittableList {
            objects: vec![objects]
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    //NOT SURE IF THIS WORKS, IF EXPERIENCING ERRORS COME BACK HERE
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, mut rec: &mut HitRecord) -> bool {

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let mut temp_rec = HitRecord::default();
            if object.hit(&ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(ray.get_direction(), *outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    pub fn default() -> HitRecord {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.,
            front_face: false
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}