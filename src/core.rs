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

#[derive(Debug, Default, Clone, Copy)]
pub struct Rectangle {
    pub min: Vector2,
    pub max: Vector2,
}

pub struct ParticleCollision {
    pub i: usize,         // index of circle A
    pub j: usize,         // index of circle B
    pub normal: Vector2,  // from B to A
    pub penetration: f64, // how much overlap
    pub v_i: Vector2,
    pub v_j: Vector2,
}

pub struct StaticCollision {
    pub index: usize,
    pub normal: Vector2,
    pub penetration: f64,
    pub velocity: Vector2,
}
