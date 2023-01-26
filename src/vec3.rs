use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Vec3 * f32
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

// Vec3 / f32
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        self * (1.0 / other)
    }
}

// f32 * Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

// Vec3 * i32
impl Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: i32) -> Vec3 {
        self * (other as f32)
    }
}

// i32 * Vec3
impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

// Vec3 / i32
impl Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: i32) -> Vec3 {
        self / (other as f32)
    }
}

// Vec3[i]
impl Index<i32> for Vec3 {
    type Output = f32;

    fn index(self: &Vec3, index: i32) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("You shouldn't have done this!"),
        }
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        self + (-1 * other)
    }
}

// Vec3 * Vec3 (dot product)
impl Mul for Vec3 {
    type Output = f32;

    fn mul(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// Vec3 x Vec3 (cross product)
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x + a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();

        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn normalized(&self) -> Vec3 {
        let len = self.length();

        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub type Point = Vec3;
pub type Color = Vec3;
