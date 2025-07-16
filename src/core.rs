use macroquad::color::Color;

use crate::vector2::Vector2;
use crate::vector2::dot;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f64,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub min: Vector2,
    pub max: Vector2,
}

pub fn reflect_with_damping(velocity: Vector2, surface_normal: Vector2) -> Vector2 {
    const RESTITUTION: f64 = 0.95;
    let normal_component = dot(velocity, surface_normal) * surface_normal;
    let tangential_component = velocity - normal_component;
    tangential_component - RESTITUTION * normal_component
}
