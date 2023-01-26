use crate::Color;

fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        return min;
    }
    if val > max {
        return max;
    }
    return val;
}

pub fn write_color(color: &mut Color, samples_per_pixel: i32) {
    *color = *color * (1.0 / samples_per_pixel as f32);

    println!(
        "{} {} {}",
        (256 as f32 * clamp(color.x, 0.0, 0.999)) as i32,
        (256 as f32 * clamp(color.y, 0.0, 0.999)) as i32,
        (256 as f32 * clamp(color.z, 0.0, 0.999)) as i32
    );
}
