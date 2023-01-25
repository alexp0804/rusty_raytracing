
mod vec3;
mod color;
mod ray;

use crate::vec3::*;
use crate::color::*;
use crate::ray::*;
use crate::Point;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: usize = 400;
const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;

fn hit_sphere(center: Point, radius: f32, ray: Ray) -> bool {
    let oc: Vec3 = ray.origin - center;
    let a: f32 = ray.direction * ray.direction;
    let b: f32 = 2.0 * oc * ray.direction;
    let c: f32 = oc * oc - radius * radius;
    let discriminant: f32 = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(new_vec(0.0, 0.0, -1.0), 0.5, ray) {
        return new_vec(1.0, 0.0, 0.0);
    }

    let unit_dir: Vec3 = ray.direction.normalized();
    let t: f32 = 0.5 * (unit_dir.y + 1.0);

    (1.0 - t) * new_vec(1.0, 1.0, 1.0) + t * new_vec(0.5, 0.7, 1.0)
}

fn main() {
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Point = new_zeros();
    let horizontal: Vec3 = new_vec(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = new_vec(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 = origin - horizontal * 0.5 - vertical * 0.5 - new_vec(0.0, 0.0, focal_length);

    print!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_HEIGHT-1).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let u: f32 = i as f32 / (IMG_WIDTH - 1) as f32;
            let v: f32 = j as f32 / (IMG_HEIGHT - 1) as f32;

            let r: Ray = new_ray(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color: Color = ray_color(r);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}