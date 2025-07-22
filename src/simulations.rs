use macroquad::color::Color;

use crate::{
    core::{Particle, Rectangle},
    impuls_sim::ImpulsSimulation,
    vector2::Vector2,
};

pub fn collision_sim() -> ImpulsSimulation {
    let color = Color::new(0.0, 0.8667, 0.8353, 1.0);

    const EPS: f64 = 1e-8;
    const RADIUS: f64 = 0.02;

    let boundary = Rectangle {
        min: Vector2 { x: 0.0, y: 0.0 },
        max: Vector2 { x: 2.0, y: 1.0 },
    };
    let spawn_bounds_x = RADIUS + boundary.min.x + EPS..boundary.max.x - RADIUS - EPS;
    let spawn_bounds_y = RADIUS + boundary.min.y + EPS..boundary.max.y - RADIUS - EPS;

    let mut particles = Vec::new();
    for _ in 0..100 {
        particles.push(Particle {
            mass: std::f64::consts::PI * RADIUS * RADIUS,
            position: Vector2::random_in_rectangle(spawn_bounds_x.clone(), spawn_bounds_y.clone()),
            velocity: Vector2::random_in_disk() * 0.5,
            radius: RADIUS,
            color: color,
        });
    }

    ImpulsSimulation {
        window_width: 800.0,
        window_height: 400.0,
        particles,
        view: boundary,
        boundary,
        gravity: Vector2 { x: 0.0, y: -0.1 },
        restitution: 1.0,
        ..Default::default()
    }
}
