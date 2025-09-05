use macroquad::color::Color;

use crate::vector2::Vector2;

#[derive(Debug, Default, Clone, Copy)]
pub struct Particle {
    pub mass: f64,
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f64,
    pub color: Color,
}

impl Particle {
    pub fn collides(self, other: &Particle) -> bool {
        (self.position - other.position).length_squared() < (self.radius + other.radius).powi(2)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Rectangle {
    pub min: Vector2,
    pub max: Vector2,
}

impl Rectangle {
    pub fn width(self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn height(self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn contains(&self, point: Vector2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}

pub struct ParticleCollision {
    pub i: usize,         // index of circle A
    pub j: usize,         // index of circle B
    pub normal: Vector2,  // from B to A
    pub penetration: f64, // how much overlap
    pub velocity1: Vector2,
    pub velocity2: Vector2,
}

pub struct StaticCollision {
    pub index: usize,
    pub normal: Vector2,
    pub penetration: f64,
    pub velocity: Vector2,
}
