use std::rc::*;

use crate::vec3::*;
use crate::hittable::*;
use crate::ray::*;
use crate::material::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Vec<Rc<dyn Material>>
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere {
            center: Point3::default(),
            radius: 0.0,
            material: vec![]
        }
    }

    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material: vec![material]
        }
    }

    pub fn add(&mut self, material: Rc<dyn Material>) {
        self.material.push(material);
    }

    pub fn clear(&mut self) {
        self.material.clear();
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.get_origin() - self.center;
        let a = ray.get_direction().length_squared();
        let half_b = Vec3::dot(oc, ray.get_direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false
        }
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
           let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.material = self.material.clone();
        true
    }
}
