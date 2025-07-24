use macroquad::prelude::*;

use crate::core::Particle;
use crate::core::Rectangle;
use crate::vector2::Vector2;

pub fn render(particle: &Vec<Particle>, view: &Rectangle) {
    for p in particle {
        draw_circle(
            to_screen(p.position, view).x as f32,
            to_screen(p.position, view).y as f32,
            (get_scale(view).x * p.radius) as f32,
            p.color,
        );
    }

    let fps_text = format!("FPS: {:.1}", get_fps());
    draw_text(&fps_text, 10.0, 20.0, 20.0, WHITE);
}

pub fn get_screen_size() -> Vector2 {
    Vector2 {
        x: screen_width() as f64,
        y: screen_height() as f64,
    }
}

pub fn get_scale(view: &Rectangle) -> Vector2 {
    let screen = get_screen_size();
    (screen * Vector2::new(1.0, -1.0)) / (view.max - view.min)
}

pub fn to_screen(p: Vector2, view: &Rectangle) -> Vector2 {
    let screen = get_screen_size();
    let s = get_scale(view);
    let t = Vector2::new(screen.x, 0.0) - s * view.max;
    s * p + t
}

// S * LB + T = (0, screen.y)
// S * RT + T = (screen.x, 0)
//
// S = screen*(1,-1)/(LB-RT);

pub fn draw_trail(view: &Rectangle, trail: &Vec<Vector2>) {
    for i in 0..trail.len().saturating_sub(1) {
        let a = to_screen(trail[i], view);
        let b = to_screen(trail[i + 1], view);

        draw_line(a.x as f32, a.y as f32, b.x as f32, b.y as f32, 1.0, RED);
    }
}
