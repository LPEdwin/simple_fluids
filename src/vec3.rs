#[derive(Debug, Clone, Copy)]
pub(crate) struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        dot(self, self).sqrt()
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

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[inline]
fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn random_in_square() -> Vec3 {
    let mut rng = rand::rng();
    Vec3 {
        y: rand::random_range(-1.0..1.0),
        x: rand::random_range(-1.0..1.0),
        z: rand::random_range(-1.0..1.0),
    }
}

fn random_in_sphere() -> Vec3 {
    loop {
        let tmp = random_in_square();
        if tmp.length() <= 1.0 {
            return tmp.normalized();
        }
    }
}
