mod core;
mod impuls_sim;
mod render;
mod simulation_information;
mod simulations;
mod vector2;

use macroquad::prelude::*;

use crate::{
    impuls_sim::ImpulsSimulation,
    render::{draw_trail, render},
    simulation_information::SimulationInformation,
};

#[macroquad::main("Simulation")]
async fn main() {
    let mut sim = simulations::brownian_motion_sim();
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
    //run_realtime(&mut sim, &mut info).await;
}

async fn run_realtime(sim: &mut ImpulsSimulation, info: &mut SimulationInformation) {
    loop {
        clear_background(BLACK);
        let dt = get_frame_time() as f64;
        sim.update(dt);
        info.update(sim, dt);

        render(&sim.particles, &sim.view);

        if let Some(trail) = info.trails.iter().nth(0) {
            draw_trail(&sim.view, trail.1)
        };

        next_frame().await;
    }
}

async fn run(sim: &mut ImpulsSimulation, fixed_dt: f64, info: &mut SimulationInformation) {
    loop {
        clear_background(BLACK);
        sim.update(fixed_dt);
        info.update(sim, fixed_dt);

        render(&sim.particles, &sim.view);

        if let Some(trail) = info.trails.iter().nth(0) {
            draw_trail(&sim.view, trail.1)
        };

        next_frame().await;
    }
}
