mod collision_sim;
mod vector2;

use macroquad::prelude::*;

use vector2::Vector2;

use crate::collision_sim::Rectangle;

fn window_conf() -> Conf {
    Conf {
        window_title: "Custom Size Window".to_owned(),
        window_width: 800,
        window_height: 400,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut sim = collision_sim::CollisionSimulation::new();
    sim.initialize();
    loop {
        clear_background(BLACK);
        sim.update(get_frame_time() as f64);
        render(&sim.circles, &sim.view);
        next_frame().await;
    }
}

pub fn render(circles: &Vec<collision_sim::Circle>, view: &Rectangle) {
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
