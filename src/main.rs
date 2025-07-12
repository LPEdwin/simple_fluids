mod vector2;

use macroquad::prelude::*;

use vector2::Vector2;
use vector2::dot;

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

static BOUNDS: Rectangle = Rectangle {
    min: Vector2 { x: 0.0, y: 0.0 },
    max: Vector2 { x: 2.0, y: 1.0 },
};

#[macroquad::main(window_conf)]
async fn main() {
    let color = Color::new(0.0, 0.8667, 0.8353, 1.0);
    let mut circles: Vec<Circle> = Vec::new();
    const RADIUS: f64 = 0.02;
    const EPS: f64 = 1e-8;
    let spawn_bounds_x = RADIUS + BOUNDS.min.x + EPS..BOUNDS.max.x - RADIUS + EPS;
    let spawn_bounds_y = RADIUS + BOUNDS.min.y + EPS..BOUNDS.max.y;

    for _ in 0..100 {
        circles.push(Circle {
            position: Vector2::random_in_rectangle(spawn_bounds_x.clone(), spawn_bounds_y.clone()),
            velocity: Vector2::random_in_disk() * 0.1,
            radius: RADIUS,
            color: color,
        });
    }

    loop {
        clear_background(BLACK);
        let gravity = Vector2 { x: 0.0, y: -0.1 };
        let dt = get_frame_time() as f64;
        for s in &mut circles {
            collision_update(s);
            s.velocity += gravity * dt;
            s.position += s.velocity * dt;
            draw_circle(
                to_screen(s.position).x as f32,
                to_screen(s.position).y as f32,
                (get_scale().x * s.radius) as f32,
                s.color,
            );
        }

        next_frame().await;
    }
}

fn collision_update(c: &mut Circle) {
    collision_update_with_bounds(c);
}

fn collision_update_with_bounds(c: &mut Circle) {
    const EPS: f64 = 1e-8;
    const R: f64 = 0.95;

    let reflect = |velocity: Vector2, normal: Vector2| -> Vector2 {
        let v_normal = dot(velocity, normal) * normal;
        let v_tangential = velocity - v_normal;
        return v_tangential - R * v_normal;
    };

    if (c.position.y - c.radius) - BOUNDS.min.y < EPS {
        c.velocity = reflect(c.velocity, Vector2::new(0.0, 1.0));
        if (c.position.y - c.radius) - BOUNDS.min.y < 0.0 {
            c.position.y = c.radius + BOUNDS.min.y;
        }
    }
    if BOUNDS.max.x - (c.position.x + c.radius) < EPS {
        c.velocity = reflect(c.velocity, Vector2::new(-1.0, 0.0));
        if BOUNDS.max.x - (c.position.x + c.radius) < 0.0 {
            c.position.x = -c.radius + BOUNDS.max.x;
        }
    }
    if (c.position.x - c.radius) - BOUNDS.min.x < EPS {
        c.velocity = reflect(c.velocity, Vector2::new(1.0, 0.0));
        if (c.position.x - c.radius) - BOUNDS.min.x < 0.0 {
            c.position.x = c.radius + BOUNDS.min.x;
        }
    }

    if c.velocity.length() < EPS {
        print!("Hit");
        c.velocity = Vector2::ZERO;
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
