mod vector2;

use macroquad::prelude::*;

use vector2::Vector2;

fn window_conf() -> Conf {
    Conf {
        window_title: "Custom Size Window".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

struct Circle {
    position: Vector2,
    direction: Vector2,
    radius: f64,
    color: Color,
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen = Vector2 {
        x: screen_width() as f64,
        y: screen_height() as f64,
    };

    let color = Color::new(0.0, 0.8667, 0.8353, 1.0);
    let mut shapes: Vec<Circle> = Vec::new();
    for _ in 0..1000 {
        shapes.push(Circle {
            position: Vector2::random_in_square(-1.0..1.0) * screen,
            direction: Vector2::random_in_disk().normalized(),
            radius: 10.0,
            color: color,
        });
    }

    loop {
        clear_background(BLACK);
        let dt = get_frame_time() as f64;
        for s in &mut shapes {
            s.position = s.position + s.direction * dt * 10.0;
            draw_circle(
                s.position.x as f32,
                s.position.y as f32,
                s.radius as f32,
                s.color,
            );
        }

        next_frame().await;
    }
}
