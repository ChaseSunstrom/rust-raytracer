use crate::ray::*;
use crate::vec3::*;
use crate::hittable::*;
use crate::utility::*;

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let mut r = pixel_color.get_x();
    let mut g = pixel_color.get_y();
    let mut b = pixel_color.get_z();

    let scale = 1.0 / samples_per_pixel as f64;

    r = (r* scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    println!("{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)),
        (256.0 * clamp(g, 0.0, 0.999)),
        (256.0 * clamp(b, 0.0, 0.999)));
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i64) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::default();
    }

    if world.hit(ray, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        if rec.material[0].scatter(ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1)
        }

        return Color::default();
    }

    let unit_direction = unit_vector(ray.get_direction());
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
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