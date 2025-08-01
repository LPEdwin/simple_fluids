mod core;
mod render;
mod simulation;
mod simulation_factory;
mod simulation_information;
mod vector2;

use macroquad::prelude::*;

use crate::{render::run, simulation_information::SimulationInformation};

#[macroquad::main("Simulation")]
async fn main() {
    let mut sim = simulation_factory::brownian_motion_sim();
    request_new_screen_size(sim.window_width, sim.window_height);
    let fixed_dt = 0.001;
    let mut info = SimulationInformation::default();
    if let Some(big) = sim
        .particles
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.radius.partial_cmp(&b.radius).unwrap())
    {
        info.trails.insert(big.0, Vec::new());
    }

    run(&mut sim, fixed_dt, &mut info).await;
}
