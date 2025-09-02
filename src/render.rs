use macroquad::prelude::*;

use crate::core::Particle;
use crate::core::Rectangle;
use crate::simulation::Simulation;
use crate::vector2::Vector2;

pub async fn run(sim: &mut Simulation, fixed_dt: f64) {
    let mut real_time_elapsed = 0.0;
    let mut simulated_time = 0.0;
    let mut pending_sim_time = 0.0;
    let mut sim_speed = 0.0;
    loop {
        clear_background(BLACK);
        let dt = get_frame_time() as f64;
        real_time_elapsed += dt;
        pending_sim_time += dt;

        while pending_sim_time >= fixed_dt {
            sim.update(fixed_dt);
            pending_sim_time -= fixed_dt;
            simulated_time += fixed_dt;
            // if below framerate limit don't simulate multiple steps
            if dt > 1.0 / 60.0 {
                break;
            }
        }

        sim_speed = simulated_time / real_time_elapsed;

        render_particles(&sim.particles, &sim.view);
        render_trails(sim);
        render_info(Some(sim_speed));

        next_frame().await;
    }
}

fn render_info(sim_speed: Option<f64>) {
    let fps_text = format!("FPS: {:.1}", get_fps());
    draw_text(&fps_text, 10.0, 20.0, 20.0, WHITE);
    if let Some(speed) = sim_speed {
        let sim_speed_text: String = format!("Speed: {:.4}", speed);
        draw_text(&sim_speed_text, 10.0, 40.0, 20.0, WHITE);
    }
}

pub async fn run_realtime(sim: &mut Simulation) {
    loop {
        clear_background(BLACK);
        let dt = get_frame_time() as f64;
        sim.update(dt);

        render_particles(&sim.particles, &sim.view);
        render_trails(sim);
        render_info(None);

        next_frame().await;
    }
}

pub fn render_particles(particle: &Vec<Particle>, view: &Rectangle) {
    for p in particle {
        draw_circle(
            to_screen(p.position, view).x as f32,
            to_screen(p.position, view).y as f32,
            (get_scale(view).x * p.radius) as f32,
            p.color,
        );
    }
}

fn get_screen_size() -> Vector2 {
    Vector2 {
        x: screen_width() as f64,
        y: screen_height() as f64,
    }
}

fn get_scale(view: &Rectangle) -> Vector2 {
    let screen = get_screen_size();
    (screen * Vector2::new(1.0, -1.0)) / (view.max - view.min)
}

fn to_screen(p: Vector2, view: &Rectangle) -> Vector2 {
    let screen = get_screen_size();
    let s = get_scale(view);
    let t = Vector2::new(screen.x, 0.0) - s * view.max;
    s * p + t
}

// S * LB + T = (0, screen.y)
// S * RT + T = (screen.x, 0)
//
// S = screen*(1,-1)/(LB-RT);

fn render_trail(view: &Rectangle, trail: &Vec<Vector2>) {
    for i in 0..trail.len().saturating_sub(1) {
        let a = to_screen(trail[i], view);
        let b = to_screen(trail[i + 1], view);

        draw_line(a.x as f32, a.y as f32, b.x as f32, b.y as f32, 1.0, RED);
    }
}

fn render_trails(sim: &mut Simulation) {
    for trail in sim.trails.values() {
        render_trail(&sim.view, trail);
    }
}
