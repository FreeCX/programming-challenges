use std::ops::{ Mul, BitXor };

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    fn new<I: Into<f64>>(x: I, y: I, z: I) -> Vec3 {
        Vec3 { x: x.into(), y: y.into(), z: z.into() }
    }
}

impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl BitXor for Vec3 {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }
}

fn main() {
    let a = Vec3::new(10, 20, 30);
    let b = Vec3::new(15, 25, 35);
    // dot operation
    let c = a * b;
    // cross operation
    let d = a ^ b;
    println!("{:?} * {:?} = {:?}", a, b, c);
    println!("{:?} * {:?} = {:?}", a, b, d);
}