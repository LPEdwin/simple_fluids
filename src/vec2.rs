#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn random_in_square(range: std::ops::Range<f64>) -> Vec2 {
        Vec2 {
            x: rand::random_range(range.clone()),
            y: rand::random_range(range),
        }
    }
}
