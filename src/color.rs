use crate::ray::*;
use crate::vec3::*;

pub fn write_color(pixel_color: Color) {
    println!("{} {} {}",
             255.999 * pixel_color.get_x(),
             255.999 * pixel_color.get_y(),
             255.999 * pixel_color.get_z());
}

pub fn ray_color(ray: &Ray) -> Color {
    let unit_direction: Vec3 = unit_vector(ray.get_direction());
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0-t) * Color::new_with_values(1.0, 1.0, 1.0) + t * Color::new_with_values(0.5, 0.7, 1.0)
}