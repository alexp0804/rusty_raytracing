mod color;
mod ray;
mod vec3;
mod hittable;
mod hittablelist;
mod sphere;

use crate::hittable::*;
use crate::hittablelist::*;
use crate::sphere::*;
use crate::ray::*;
use crate::vec3::*;
use crate::color::*;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 400;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
const INFINITY: f32 = f32::INFINITY;

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
    // Image
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // // Camera
    let origin: Point = Vec3::origin();
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 =
        origin - horizontal * 0.5 - vertical * 0.5 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_HEIGHT - 1).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let u = i as f32 / (IMG_WIDTH - 1) as f32;
            let v = j as f32 / (IMG_HEIGHT - 1) as f32;

            let r: Ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color: Color = ray_color(r, &world);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}
