use std::collections::HashMap;

use crate::{impuls_sim::ImpulsSimulation, vector2::Vector2};

#[derive(Debug, Default)]
pub struct SimulationInformation {
    pub trails: HashMap<usize, Vec<Vector2>>,
}

impl SimulationInformation {
    pub fn update(&mut self, sim: &mut ImpulsSimulation, fixed_dt: f64) {
        for (index, trail) in &mut self.trails {
            let p = sim.particles[*index];
            trail.push(p.position);
            if trail.len() > 500 {
                trail.remove(0);
            }
        }
    }
}
