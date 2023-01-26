mod camera;
mod color;
mod hittable;
mod hittablelist;
mod ray;
mod sphere;
mod vec3;

use crate::camera::*;
use crate::color::*;
use crate::hittable::*;
use crate::hittablelist::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;
use rand::prelude::*;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 400;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: i32 = 100;
const VIEWPORT_HEIGHT: f32 = 2.0;
const FOCAL_LENGTH: f32 = 1.0;

const INFINITY: f32 = f32::INFINITY;

fn random_f32(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();

    min + (max - min) * rng.gen::<f32>()
}

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::new_empty();
    if world.hit(ray, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_dir = ray.direction.normalized();
    let t = 0.5 * (unit_dir.y + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new(ASPECT_RATIO, VIEWPORT_HEIGHT, FOCAL_LENGTH);

    // Render
    print!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);
    let mut rng = rand::thread_rng();

    for j in (0..IMG_HEIGHT - 1).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let mut pixel_color: Color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (IMG_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (IMG_HEIGHT - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world);
            }
            write_color(&mut pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone.");
}
