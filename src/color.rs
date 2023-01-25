
use crate::Color;

pub fn write_color(color: Color) {
    println!("{} {} {}",
        (color[0] * 255.99) as i32,
        (color[1] * 255.99) as i32,
        (color[2] * 255.99) as i32
    );
}