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

pub fn mixing_sim() -> ImpulsSimulation {
    let color1 = Color::new(0.0, 0.8667, 0.8353, 1.0);
    let color2 = Color::new(0.9254, 0.0745, 0.2745, 1.0);

    const EPS: f64 = 1e-8;
    const RADIUS: f64 = 0.01;
    const COUNT: usize = 1000;

    let boundary = Rectangle {
        min: Vector2 { x: 0.0, y: 0.0 },
        max: Vector2 { x: 1.0, y: 2.0 },
    };
    let spawn_bounds_x = RADIUS + boundary.min.x + EPS..boundary.max.x - RADIUS - EPS;
    let spawn_bounds_y1 = RADIUS + boundary.min.y + EPS..boundary.max.y / 2.0 - RADIUS;
    let spawn_bounds_y2 = RADIUS + boundary.max.y / 2.0 + EPS..boundary.max.y - RADIUS;

    let mut particles = Vec::new();
    for _ in 0..COUNT {
        particles.push(Particle {
            mass: std::f64::consts::PI * RADIUS * RADIUS,
            position: Vector2::random_in_rectangle(spawn_bounds_x.clone(), spawn_bounds_y1.clone()),
            velocity: Vector2::random_in_disk() * 0.5,
            radius: RADIUS,
            color: color1,
        });
    }

    for _ in 0..COUNT {
        particles.push(Particle {
            mass: std::f64::consts::PI * RADIUS * RADIUS,
            position: Vector2::random_in_rectangle(spawn_bounds_x.clone(), spawn_bounds_y2.clone()),
            velocity: Vector2::random_in_disk() * 0.5,
            radius: RADIUS,
            color: color2,
        });
    }

    ImpulsSimulation {
        window_width: 400.0,
        window_height: 800.0,
        particles,
        view: boundary,
        boundary,
        gravity: Vector2 { x: 0.0, y: -0.0 },
        restitution: 1.0,
        ..Default::default()
    }
}
