mod vec3;
mod color;
mod ray;
mod hittable;
mod object;
mod utility;

use std::fs::File;
use std::rc::Rc;
use image::{RgbImage, Rgb, ImageBuffer};
use crate::vec3::{Point3, Vec3};
use crate::ray::*;
use crate::color::*;
use crate::hittable::*;
use crate::object::*;
use crate::utility::*;

const IMAGE_WIDTH: f64 = 1024.0;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;

fn main() {

    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    let mut world = HittableList::new();
    world.add(
        Rc::new(
            Sphere::new_with_values(
                Point3::new_with_values(0., 0., -1.), 0.5)
        ));
    world.add(
        Rc::new(
            Sphere::new_with_values(
                Point3::new_with_values(0., -100.5, -1.), 100.)
        ));

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3::new_with_values(viewport_width, 0., 0.);
    let vertical = Vec3::new_with_values(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new_with_values(0., 0., focal_length);


    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let u = x as f64 / (IMAGE_WIDTH - 1.);
        let v = 1.0 - y as f64 / (IMAGE_HEIGHT - 1.);
        let ray_direction = lower_left_corner + u * horizontal + v * vertical - origin;
        let ray = Ray::new_with_values(origin, ray_direction);
        let pixel_color = ray_color(&ray, &world);

        let ir = (255.999 * pixel_color.get_x()) as u8;
        let ig = (255.999 * pixel_color.get_y()) as u8;
        let ib = (255.999 * pixel_color.get_z()) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    buffer.save("raytracer.png").unwrap();
}
