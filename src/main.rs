mod core;
mod render;
mod simulation;
mod simulation_factory;
mod vector2;

use macroquad::prelude::*;

use crate::render::run;

#[macroquad::main("Simulation")]
async fn main() {
    let mut sim = simulation_factory::brownian_motion_sim();
    request_new_screen_size(sim.window_width, sim.window_height);
    let fixed_dt = 0.001;
    run(&mut sim, fixed_dt).await;
}
