use crate::core::Particle;
use crate::core::ParticleCollision;
use crate::core::Rectangle;
use crate::core::StaticCollision;
use crate::vector2::Vector2;
use crate::vector2::dot;
use macroquad::prelude::*;

pub struct ImpulsSimulation {
    pub window_width: f32,
    pub window_height: f32,
    pub view: Rectangle,
    pub particles: Vec<Particle>,
    pub boundary: Rectangle,
    pub gravity: Vector2,
    pub restitution: f64,
}

impl ImpulsSimulation {
    pub fn new() -> Self {
        ImpulsSimulation {
            window_width: 800.0,
            window_height: 400.0,
            particles: Vec::new(),
            view: Rectangle {
                min: Vector2 { x: 0.0, y: 0.0 },
                max: Vector2 { x: 2.0, y: 1.0 },
            },
            boundary: Rectangle {
                min: Vector2 { x: 0.0, y: 0.0 },
                max: Vector2 { x: 2.0, y: 1.0 },
            },
            gravity: Vector2 { x: 0.0, y: -0.1 },
            restitution: 1.0,
        }
    }

    pub fn initialize(&mut self) {
        let color = Color::new(0.0, 0.8667, 0.8353, 1.0);
        self.particles = Vec::new();
        const RADIUS: f64 = 0.02;
        const EPS: f64 = 1e-8;
        let boundary = self.boundary;
        let spawn_bounds_x = RADIUS + boundary.min.x + EPS..boundary.max.x - RADIUS - EPS;
        let spawn_bounds_y = RADIUS + boundary.min.y + EPS..boundary.max.y - RADIUS - EPS;

        for _ in 0..100 {
            self.particles.push(Particle {
                mass: std::f64::consts::PI * RADIUS * RADIUS,
                position: Vector2::random_in_rectangle(
                    spawn_bounds_x.clone(),
                    spawn_bounds_y.clone(),
                ),
                velocity: Vector2::random_in_disk() * 0.5,
                radius: RADIUS,
                color: color,
            });
        }
    }

    pub fn update(&mut self, dt: f64) {
        // apply gravity
        for s in &mut self.particles {
            s.velocity += self.gravity * dt;
            s.position += s.velocity * dt;
        }
        // detect collisions
        let p_collisions = detect_particle_collissions(&self.particles);
        let s_collisions = detect_static_collissions(&self.particles, &self.boundary);

        // resolve collisions
        resolve_particle_collisions(&mut self.particles, &p_collisions, self.restitution);
        resolve_and_correct_static_collisions(&mut self.particles, &s_collisions, self.restitution);

        // Todo: correct positions
    }
}

fn detect_particle_collissions(particles: &Vec<Particle>) -> Vec<ParticleCollision> {
    let mut collisions = Vec::new();

    for i in 0..particles.len() {
        for j in i + 1..particles.len() {
            let p1 = particles[i];
            let p2 = particles[j];
            let n = p1.position - p2.position;
            let d = n.length();
            if d <= p1.radius + p2.radius {
                collisions.push(ParticleCollision {
                    i,
                    j,
                    normal: n.normalized(),
                    penetration: p1.radius + p2.radius - d,
                    v_i: p1.velocity,
                    v_j: p2.velocity,
                });
            }
        }
    }
    return collisions;
}

fn resolve_particle_collisions(
    particles: &mut [Particle],
    collisions: &[ParticleCollision],
    restitution: f64,
) {
    for coll in collisions {
        let (i, j) = (coll.i, coll.j);
        unsafe {
            let p1 = particles.get_unchecked_mut(i) as *mut Particle;
            let p2 = particles.get_unchecked_mut(j) as *mut Particle;
            add_impulse(&mut *p1, &mut *p2, restitution);
        }
    }
}

fn add_impulse(p1: &mut Particle, p2: &mut Particle, restitution: f64) {
    let n = (p2.position - p1.position).normalized();
    // velocity from p1 relative to p2 (p2 is a fixed point)
    let rel = p1.velocity - p2.velocity;
    let vel_along = dot(rel, n);
    if vel_along <= 0.0 {
        // moving away from p2
        return;
    }

    let mi = p1.mass;
    let mj = p2.mass;
    let mu = mi * mj / (mi + mj);

    let j_impulse = (1.0 + restitution) * mu * vel_along;

    p1.velocity -= n * (j_impulse / mi);
    p2.velocity += n * (j_impulse / mj);
}

fn detect_static_collissions(particles: &[Particle], boundary: &Rectangle) -> Vec<StaticCollision> {
    let mut collisions = Vec::new();

    for (index, p) in particles.iter().enumerate() {
        // top
        if p.position.y + p.radius > boundary.max.y {
            collisions.push(StaticCollision {
                index,
                normal: Vector2::new(0.0, -1.0),
                penetration: (p.position.y + p.radius) - boundary.max.y,
                velocity: p.velocity,
            });
        }

        // right
        if p.position.x + p.radius > boundary.max.x {
            collisions.push(StaticCollision {
                index,
                normal: Vector2::new(-1.0, 0.0),
                penetration: (p.position.x + p.radius) - boundary.max.x,
                velocity: p.velocity,
            });
        }

        // bottom
        if p.position.y - p.radius < boundary.min.y {
            let normal = Vector2::new(0.0, 1.0);
            collisions.push(StaticCollision {
                index,
                normal,
                penetration: boundary.min.y - (p.position.y - p.radius),
                velocity: p.velocity,
            });
        }

        // left
        if p.position.x - p.radius < boundary.min.x {
            collisions.push(StaticCollision {
                index,
                normal: Vector2::new(1.0, 0.0),
                penetration: boundary.min.x - (p.position.x - p.radius),
                velocity: p.velocity,
            });
        }
    }
    return collisions;
}

// Restitution is a value from 0 to 1; 1 means perfectly elastic (no energy loss), 0 means perfectly inelastic.
fn resolve_and_correct_static_collisions(
    particles: &mut [Particle],
    collisions: &[StaticCollision],
    restitution: f64,
) {
    for c in collisions {
        let p = &mut particles[c.index];
        let n = dot(c.normal, c.velocity) * c.normal;
        p.velocity -= (1.0 + restitution) * n;
        p.position += c.normal * c.penetration;
    }
}
