use crate::utility::*;
use crate::hittable::*;
use crate::object::*;
use crate::vec3::*;
use crate::color::*;
use crate::ray::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Lambertian {
        Lambertian {
            albedo: *albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: &Color) -> Metal {
        Metal {
            albedo: *albedo
        }
    }
}

impl Material for Metal {
     fn scatter(&self, mut ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&mut unit_vector(ray_in.get_direction()), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        Vec3::dot(scattered.get_direction(), rec.normal) > 0.
    }
}