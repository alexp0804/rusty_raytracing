use crate::Color;

pub fn write_color(color: Color) {
    println!(
        "{} {} {}",
        (color.x * 255.99) as i32,
        (color.y * 255.99) as i32,
        (color.z * 255.99) as i32
    );
}
