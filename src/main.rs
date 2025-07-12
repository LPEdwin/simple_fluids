mod vector2;

use macroquad::prelude::*;

use vector2::Vector2;

fn window_conf() -> Conf {
    Conf {
        window_title: "Custom Size Window".to_owned(),
        window_width: 800,
        window_height: 400,
        ..Default::default()
    }
}

struct Circle {
    position: Vector2,
    velocity: Vector2,
    radius: f64,
    color: Color,
}

struct Rectangle {
    min: Vector2,
    max: Vector2,
}

static VIEW: Rectangle = Rectangle {
    min: Vector2 { x: 0.0, y: 0.0 },
    max: Vector2 { x: 2.0, y: 1.0 },
};

#[macroquad::main(window_conf)]
async fn main() {
    let color = Color::new(0.0, 0.8667, 0.8353, 1.0);
    let mut shapes: Vec<Circle> = Vec::new();
    for _ in 0..1000 {
        shapes.push(Circle {
            position: Vector2::random_in_rectangle(0.0..2.0, 0.0..1.0),
            velocity: Vector2::random_in_disk() * 0.1,
            radius: get_scale().x * 0.02,
            color: color,
        });
    }

    loop {
        clear_background(BLACK);
        let gravity = Vector2 { x: 0.0, y: -0.1 };
        let dt = get_frame_time() as f64;
        for s in &mut shapes {
            s.velocity += gravity * dt;
            s.position += s.velocity * dt;
            draw_circle(
                to_screen(s.position).x as f32,
                to_screen(s.position).y as f32,
                s.radius as f32,
                s.color,
            );
        }

        next_frame().await;
    }
}

fn get_screen_size() -> Vector2 {
    Vector2 {
        x: screen_width() as f64,
        y: screen_height() as f64,
    }
}

fn get_scale() -> Vector2 {
    let screen = get_screen_size();
    (screen * Vector2::new(1.0, -1.0)) / (VIEW.max - VIEW.min)
}

fn to_screen(p: Vector2) -> Vector2 {
    let screen = get_screen_size();
    let s = get_scale();
    let t = Vector2::new(screen.x, 0.0) - s * VIEW.max;
    s * p + t
}

// S * LB + T = (0, screen.y)
// S * RT + T = (screen.x, 0)
//
// S = screen*(1,-1)/(LB-RT);
