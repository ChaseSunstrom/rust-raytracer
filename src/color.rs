use crate::ray::*;
use crate::vec3::*;

pub fn write_color(pixel_color: Color) {
    println!("{} {} {}",
             255.999 * pixel_color.get_x(),
             255.999 * pixel_color.get_y(),
             255.999 * pixel_color.get_z());
}

pub fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point3::new_with_values(0., 0., -1.), 0.5, ray);
    if t > 0.0 {
        let N = unit_vector(ray.at(t) - Vec3::new_with_values(0., 0., -1.));
        return 0.5 * Color::new_with_values(
            N.get_x() + 1.,
            N.get_y() + 1.,
            N.get_z() + 1.
        )
    }
    let unit_direction: Vec3 = unit_vector(ray.get_direction());
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0-t) * Color::new_with_values(1.0, 1.0, 1.0) + t * Color::new_with_values(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.get_origin() - *center;
    let a = Vec3::dot(ray.get_direction(), ray.get_direction());
    let b = 2.0 * Vec3::dot(oc, ray.get_direction());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }

}