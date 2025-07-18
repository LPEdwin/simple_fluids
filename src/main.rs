mod collision_sim;
mod core;
mod impuls_sim;
mod vector2;

use macroquad::prelude::*;

use crate::core::Particle;
use crate::core::Rectangle;
use crate::impuls_sim::ImpulsSimulation;
use crate::vector2::Vector2;

#[macroquad::main("Simulation")]
async fn main() {
    //let mut sim = collision_sim::CollisionSimulation::new();
    let mut sim = ImpulsSimulation::new();
    sim.initialize();

    request_new_screen_size(sim.window_width, sim.window_height);

    loop {
        clear_background(BLACK);
        sim.update(get_frame_time() as f64);
        render(&sim.circles, &sim.view);
        next_frame().await;
    }
}

pub fn render(circles: &Vec<Particle>, view: &Rectangle) {
    for s in circles {
        draw_circle(
            to_screen(s.position, view).x as f32,
            to_screen(s.position, view).y as f32,
            (get_scale(view).x * s.radius) as f32,
            s.color,
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
