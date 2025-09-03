use std::collections::HashMap;

use macroquad::color::Color;
use rand_pcg::Pcg64Mcg;

use crate::{
    core::{Particle, Rectangle},
    simulation::Simulation,
    vector2::Vector2,
};

const GREEN: Color = Color::new(0.0, 0.8667, 0.8353, 1.0);
const RED: Color = Color::new(0.9254, 0.0745, 0.2745, 1.0);

pub fn collision_sim() -> Simulation {
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
            color: GREEN,
        });
    }

    Simulation {
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

pub fn mixing_sim() -> Simulation {
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
            color: GREEN,
        });
    }

    for _ in 0..COUNT {
        particles.push(Particle {
            mass: std::f64::consts::PI * RADIUS * RADIUS,
            position: Vector2::random_in_rectangle(spawn_bounds_x.clone(), spawn_bounds_y2.clone()),
            velocity: Vector2::random_in_disk() * 0.5,
            radius: RADIUS,
            color: RED,
        });
    }

    Simulation {
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

pub fn brownian_motion_sim() -> Simulation {
    const EPS: f64 = 1e-8;
    const RADIUS: f64 = 0.005;
    const MASS: f64 = 1.0;

    let boundary = Rectangle {
        min: Vector2 { x: 0.0, y: 0.0 },
        max: Vector2 { x: 2.0, y: 1.0 },
    };
    let spawn_bounds_x = RADIUS + boundary.min.x + EPS..boundary.max.x - RADIUS - EPS;
    let spawn_bounds_y = RADIUS + boundary.min.y + EPS..boundary.max.y - RADIUS - EPS;

    const KB: f64 = 1.0; // Boltzmann constant, normalized
    const T: f64 = 1.0; // arbitrary temperature
    let sigma = (KB * T / MASS).sqrt();

    let mut particles = Vec::new();
    for _ in 0..1000 {
        particles.push(Particle {
            mass: MASS,
            position: Vector2::random_in_rectangle(spawn_bounds_x.clone(), spawn_bounds_y.clone()),
            velocity: Vector2::random_gaussian(0.0, sigma),
            radius: RADIUS,
            color: GREEN,
        });
    }

    let big_p = Particle {
        mass: MASS * 100.0,
        position: Vector2::random_in_rectangle(spawn_bounds_x.clone(), spawn_bounds_y.clone()),
        velocity: Vector2::ZERO,
        radius: RADIUS * 10.0,
        color: RED,
    };
    particles.push(big_p);
    let mut trails: HashMap<usize, Vec<Vector2>> = HashMap::new();
    trails.insert(particles.len() - 1, Vec::new());

    Simulation {
        window_width: 500.0,
        window_height: 500.0,
        particles,
        view: boundary,
        boundary,
        gravity: Vector2::ZERO,
        restitution: 1.0,
        trails,
        ..Default::default()
    }
}

use rand::{Rng, SeedableRng};

/// Randomly generates `count` non-overlapping particles inside `boundary`.
/// Each particle has the given `radius`, and up to `max_attempts_per_particle` trials are made.
fn generate_non_overlapping_particles(
    boundary: Rectangle,
    radius: f64,
    count: usize,
    max_attempts_per_particle: usize,
    color: Color,
) -> Vec<Particle> {
    let mut rng = Pcg64Mcg::from_rng(&mut rand::rng());
    let mut particles = Vec::with_capacity(count);
    let mut grid = crate::uniform_grid::UniformGrid::with_cell_size(boundary, 2.0 * radius);

    while particles.len() < count {
        let mut placed = false;

        for _ in 0..max_attempts_per_particle {
            let x = rng.random_range(boundary.min.x + radius..=boundary.max.x - radius);
            let y = rng.random_range(boundary.min.y + radius..=boundary.max.y - radius);
            let position = Vector2 { x, y };

            let candidate = Particle {
                position,
                radius,
                velocity: Vector2::ZERO,
                mass: std::f64::consts::PI * radius * radius,
                color,
            };

            // Check local neighbors via uniform grid
            let neighbors = grid.get_close_colliders(&candidate);
            let mut overlaps = false;
            for &j in &neighbors {
                let other: &Particle = &particles[j];
                let dist_sq = (candidate.position - other.position).length_squared();
                if dist_sq < (2.0 * radius).powi(2) {
                    overlaps = true;
                    break;
                }
            }

            if !overlaps {
                let idx = particles.len();
                particles.push(candidate);
                grid.add_particle(idx, &candidate);
                placed = true;
                break;
            }
        }

        if !placed {
            eprintln!(
                "Warning: only placed {} out of {} particles due to overlap constraints.",
                particles.len(),
                count
            );
            break;
        }
    }

    particles
}
