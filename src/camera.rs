use crate::vec3::*;
use crate::hittable::*;
use crate::object::*;
use crate::utility::*;
use crate::ray::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn default() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        Camera {
            origin: Point3::default(),
            horizontal: Vec3::new(viewport_width, 0., 0.),
            vertical: Vec3::new(0., viewport_height, 0.),
            lower_left_corner: Point3::default() - Vec3::new(viewport_width, 0.0, 0.0) / 2.0 - Vec3::new(0.0, viewport_height, 0.0) / 2.0 - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }

}