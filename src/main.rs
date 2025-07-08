use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    let v1: Vector3 = Vector3 {
        x: 3.14,
        y: 55.0,
        z: 0.55,
    };
    let v2: Vector3 = Vector3 {
        x: 0.0,
        y: 1000.0,
        z: 0.55,
    };

    println!("{:?}", v1 + v2);
}
