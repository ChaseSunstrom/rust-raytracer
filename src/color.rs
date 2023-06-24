use crate::ray::*;
use crate::vec3::*;
use crate::hittable::*;
use crate::utility::*;
use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    println!("{} {} {}",
             255.999 * pixel_color.get_x(),
             255.999 * pixel_color.get_y(),
             255.999 * pixel_color.get_z());
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(ray, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new_with_values(1.0, 1.0, 1.0));
    }
    let unit_direction = unit_vector(ray.get_direction());
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0 - t) * Color::new_with_values(1.0, 1.0, 1.0) + t * Color::new_with_values(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.get_origin() - *center;
    let a = ray.get_direction().length_squared();
    let half_b = Vec3::dot(oc, ray.get_direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }

}