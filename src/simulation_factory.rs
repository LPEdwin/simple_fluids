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
    const RADIUS: f64 = 0.02;

    let mut rng = Pcg64Mcg::from_rng(&mut rand::rng());

    let boundary = Rectangle {
        min: Vector2 { x: 0.0, y: 0.0 },
        max: Vector2 { x: 2.0, y: 1.0 },
    };

    let mut particles =
        generate_non_overlapping_particles(boundary, RADIUS, 100, 5, GREEN, &mut rng);
    for p in &mut particles {
        p.mass = std::f64::consts::PI * RADIUS * RADIUS;
        p.velocity = Vector2::random_in_disk() * 0.5;
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
    const RADIUS: f64 = 0.01;
    const COUNT: usize = 300;

    let mut rng = Pcg64Mcg::from_rng(&mut rand::rng());

    let boundary = Rectangle {
        min: Vector2 { x: 0.0, y: 0.0 },
        max: Vector2 { x: 1.0, y: 2.0 },
    };

    let top_boundary = Rectangle {
        min: Vector2 { x: 0.0, y: 1.0 },
        max: Vector2 { x: 1.0, y: 2.0 },
    };

    let top_particles =
        generate_non_overlapping_particles(top_boundary, RADIUS, COUNT, 5, GREEN, &mut rng);

    let bottom_boundary = Rectangle {
        min: Vector2 { x: 0.0, y: 0.0 },
        max: Vector2 { x: 1.0, y: 1.0 },
    };

    let bottom_particles =
        generate_non_overlapping_particles(bottom_boundary, RADIUS, COUNT, 5, RED, &mut rng);

    let mut particles = top_particles;
    particles.extend(bottom_particles);

    for p in &mut particles {
        p.mass = std::f64::consts::PI * RADIUS * RADIUS;
        p.velocity = Vector2::random_in_disk() * 0.5;
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
    const RADIUS: f64 = 0.005;
    const BIG_RADIUS: f64 = RADIUS * 10.0;
    const MASS: f64 = 1.0;
    const COUNT: usize = 1000;

    let mut rng = Pcg64Mcg::from_rng(&mut rand::rng());

    let boundary = Rectangle {
        min: Vector2 { x: 0.0, y: 0.0 },
        max: Vector2 { x: 2.0, y: 1.0 },
    };

    // Generate small particles using generate_non_overlapping_particles
    let mut particles =
        generate_non_overlapping_particles(boundary, RADIUS, COUNT, 10, GREEN, &mut rng);

    // Set masses and velocities for small particles
    const KB: f64 = 1.0; // Boltzmann constant, normalized
    const T: f64 = 1.0; // arbitrary temperature
    let sigma = (KB * T / MASS).sqrt();

    for p in &mut particles {
        p.mass = MASS;
        p.velocity = Vector2::random_gaussian(0.0, sigma);
    }

    // Generate big particle, ensuring no overlap with small particles
    let mut big_particle_placed = false;
    let mut big_p = Particle {
        mass: MASS * 100.0,
        position: Vector2::ZERO, // Will be set
        velocity: Vector2::ZERO,
        radius: BIG_RADIUS,
        color: RED,
    };

    // Create a grid for overlap checking with the big particle
    let mut grid = crate::uniform_grid::UniformGrid::with_cell_size(boundary, 2.0 * BIG_RADIUS);
    for (i, p) in particles.iter().enumerate() {
        grid.add_particle(i, p);
    }

    for _ in 0..1000 {
        // Try up to 100 times to place big particle
        let x = rng.random_range(boundary.min.x + BIG_RADIUS..=boundary.max.x - BIG_RADIUS);
        let y = rng.random_range(boundary.min.y + BIG_RADIUS..=boundary.max.y - BIG_RADIUS);
        big_p.position = Vector2 { x, y };

        // Check for overlaps with small particles
        let neighbors = grid.get_close_colliders(&big_p);
        let mut overlaps = false;
        for &j in &neighbors {
            let other = &particles[j];
            let dist_sq = (big_p.position - other.position).length_squared();
            if dist_sq < (BIG_RADIUS + other.radius).powi(2) {
                overlaps = true;
                break;
            }
        }

        if !overlaps {
            big_particle_placed = true;
            break;
        }
    }

    if !big_particle_placed {
        panic!("Error: Could not place big particle without overlapping small particles.");
    }

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
    particle_radius: f64,
    count: usize,
    max_attempts_per_particle: usize,
    color: Color,
    rng: &mut impl Rng,
) -> Vec<Particle> {
    let mut particles = Vec::with_capacity(count);
    let mut grid =
        crate::uniform_grid::UniformGrid::with_cell_size(boundary, 2.0 * particle_radius);

    while particles.len() < count {
        let mut placed = false;

        for _ in 0..max_attempts_per_particle {
            let x = rng
                .random_range(boundary.min.x + particle_radius..=boundary.max.x - particle_radius);
            let y = rng
                .random_range(boundary.min.y + particle_radius..=boundary.max.y - particle_radius);
            let position = Vector2 { x, y };

            let candidate = Particle {
                position,
                radius: particle_radius,
                velocity: Vector2::ZERO,
                mass: std::f64::consts::PI * particle_radius * particle_radius,
                color,
            };

            // Check local neighbors via uniform grid
            let neighbors = grid.get_close_colliders(&candidate);
            let mut overlaps = false;
            for &j in &neighbors {
                let other: &Particle = &particles[j];
                let dist_sq = (candidate.position - other.position).length_squared();
                if dist_sq < (2.0 * particle_radius).powi(2) {
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
