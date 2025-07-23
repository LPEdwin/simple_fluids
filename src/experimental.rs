use std::vec;

use crate::{
    core::{Particle, ParticleCollision},
    vector2::dot,
};

// unproven to actually function
// if using this, you need to remove the add impulse methods
pub fn correct_positions_baumgarte(
    bodies: &mut [Particle],
    collisions: &[ParticleCollision],
    restitution: f64,
    dt: f64,
    iterations: usize,
) {
    for _ in 0..iterations {
        solve_contacts_pgs_baumgarte(bodies, collisions, restitution, dt);
    }
}

fn solve_contacts_pgs_baumgarte(
    bodies: &mut [Particle],
    collisions: &[ParticleCollision],
    restitution: f64,
    dt: f64,
) {
    const MAX_BAUMGARTE_VELOCITY: f64 = 4.0;
    const BAUMGARTE: f64 = 0.2;
    const LINEAR_SLOP: f64 = 0.005;

    let inv_h = 1.0 / dt;
    let mut accu_normal_impulse = vec![0.0; collisions.len()];

    for (index, coll) in collisions.iter().enumerate() {
        let (i, j) = if coll.i < coll.j {
            (coll.i, coll.j)
        } else {
            (coll.j, coll.i)
        };
        let (left, right) = bodies.split_at_mut(j);
        let body_a = &mut left[i];
        let body_b = &mut right[0];

        let m_a = 1.0 / body_a.mass;
        let m_b = 1.0 / body_b.mass;

        let mut v_a = body_a.velocity;
        let mut v_b = body_b.velocity;

        let normal_mass = 1.0 / (m_a + m_b);

        let normal = coll.normal;
        let separation = -coll.penetration;
        let bias;
        if separation > 0.0 {
            // Speculative
            bias = separation * inv_h;
        } else {
            bias = (BAUMGARTE * inv_h * (0.0 as f64).min(separation + LINEAR_SLOP))
                .max(-MAX_BAUMGARTE_VELOCITY);
        }

        // Relative velocity at contact (no angular component)
        let vn = dot(v_b - v_a, normal);

        // Compute normal impulse
        let mut impulse = -(1.0 + restitution) * normal_mass * (vn + bias);

        // Clamp the accumulated impulse
        let new_impulse = (accu_normal_impulse[index] + impulse).max(0.0);
        impulse = new_impulse - accu_normal_impulse[index];
        accu_normal_impulse[index] = new_impulse;

        // Apply contact impulse
        let p = impulse * normal;
        v_a = v_a - m_a * p;
        v_b = v_b + m_b * p;

        body_a.velocity = v_a;
        body_b.velocity = v_b;
    }
}
