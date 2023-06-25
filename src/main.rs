mod vec3;
mod color;
mod ray;
mod hittable;
mod object;
mod utility;
mod camera;

use std::fs::File;
use std::rc::Rc;
use crate::vec3::{Color, Point3, Vec3};
use crate::ray::*;
use crate::color::*;
use crate::hittable::*;
use crate::object::*;
use crate::utility::*;
use crate::camera::*;

const IMAGE_WIDTH: f64 = 512.0;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: i64 = 50;

fn main() {

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let camera = Camera::default();

    let mut world = HittableList::default();
    world.add(
        Rc::new(
            Sphere::new(
                Point3::new(0., 0., -1.), 0.5)
        ));
    world.add(
        Rc::new(
            Sphere::new(
                Point3::new(0., -100.5, -1.), 100.)
        ));

    for y in (0..IMAGE_HEIGHT as i64).rev() {
        eprintln!("Scanlines remaining: {}", y);
        for x in 0..IMAGE_WIDTH as i64 {
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + random_float()) / (IMAGE_WIDTH - 1.);
                let v = (y as f64 + random_float()) / (IMAGE_HEIGHT - 1.);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
