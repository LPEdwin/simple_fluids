mod core;
mod render;
mod simulation;
mod simulation_factory;
mod simulation_information;
mod vector2;

use macroquad::prelude::*;

use crate::{
    render::{draw_trail, render},
    simulation::Simulation,
    simulation_information::SimulationInformation,
};

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
    //run_realtime(&mut sim, &mut info).await;
}

async fn run_realtime(sim: &mut Simulation, info: &mut SimulationInformation) {
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

async fn run(sim: &mut Simulation, fixed_dt: f64, info: &mut SimulationInformation) {
    let mut real_time_elapsed = 0.0;
    let mut simulated_time = 0.0;
    let mut pending_sim_time = 0.0;
    loop {
        clear_background(BLACK);
        let dt = get_frame_time() as f64;
        real_time_elapsed += dt;
        pending_sim_time += dt;

        while pending_sim_time >= fixed_dt {
            sim.update(fixed_dt);
            info.update(sim, fixed_dt);
            pending_sim_time -= fixed_dt;
            simulated_time += fixed_dt;
            // if below framerate limit don't simulate multiple steps
            if dt > 1.0 / 60.0 {
                break;
            }
        }

        info.simulation_speed = simulated_time / real_time_elapsed;
        render(&sim.particles, &sim.view);

        let sim_speed_text = format!("Speed: {:.4}", info.simulation_speed);
        draw_text(&sim_speed_text, 10.0, 40.0, 20.0, WHITE);
        for trail in info.trails.values() {
            draw_trail(&sim.view, trail);
        }

        next_frame().await;
    }
}
