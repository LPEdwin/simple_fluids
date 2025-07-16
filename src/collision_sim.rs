use crate::vector2::Vector2;
use crate::vector2::dot;
use macroquad::prelude::*;

pub struct Circle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f64,
    pub color: Color,
}

pub struct Rectangle {
    pub min: Vector2,
    pub max: Vector2,
}

pub static BOUNDS: Rectangle = Rectangle {
    min: Vector2 { x: 0.0, y: 0.0 },
    max: Vector2 { x: 2.0, y: 1.0 },
};

pub struct CollisionSimulation {
    pub window_width: f32,
    pub window_height: f32,
    pub view: Rectangle,
    pub circles: Vec<Circle>,
}

impl CollisionSimulation {
    pub fn new() -> Self {
        CollisionSimulation {
            window_width: 800.0,
            window_height: 400.0,
            circles: Vec::new(),
            view: Rectangle {
                min: Vector2 { x: 0.0, y: 0.0 },
                max: Vector2 { x: 2.0, y: 1.0 },
            },
        }
    }

    pub fn initialize(&mut self) {
        let color = Color::new(0.0, 0.8667, 0.8353, 1.0);
        self.circles = Vec::new();
        const RADIUS: f64 = 0.02;
        const EPS: f64 = 1e-8;
        let spawn_bounds_x = RADIUS + BOUNDS.min.x + EPS..BOUNDS.max.x - RADIUS + EPS;
        let spawn_bounds_y = RADIUS + BOUNDS.min.y + EPS..BOUNDS.max.y;

        for _ in 0..100 {
            self.circles.push(Circle {
                position: Vector2::random_in_rectangle(
                    spawn_bounds_x.clone(),
                    spawn_bounds_y.clone(),
                ),
                velocity: Vector2::random_in_disk() * 0.1,
                radius: RADIUS,
                color: color,
            });
        }
    }

    pub fn update(&mut self, dt: f64) {
        let gravity = Vector2 { x: 0.0, y: -0.1 };
        circle_collission(&mut self.circles);
        for s in &mut self.circles {
            collision_update_with_bounds(s);
            s.velocity += gravity * dt;
            s.position += s.velocity * dt;
        }
    }
}

fn circle_collission(circles: &mut Vec<Circle>) {
    let mut collisions = Vec::new();

    let collide = |c1: &Circle, c2: &Circle| {
        let d = (c1.position - c2.position).length();
        return d <= c1.radius + c2.radius;
    };

    for i in 0..circles.len() {
        for j in i + 1..circles.len() {
            if collide(&circles[i], &circles[j]) {
                collisions.push((i, j));
            }
        }
    }

    let resolve = |circles: &mut Vec<Circle>, i: usize, j: usize| {
        let n = (circles[j].position - circles[i].position).normalized();
        let rel_vel2 = circles[j].velocity - circles[i].velocity;
        if dot(rel_vel2, n) >= 0.0 {
            return;
        }
        circles[i].velocity = reflect_with_loss(circles[i].velocity, n);
        circles[j].velocity = reflect_with_loss(circles[j].velocity, -n);
    };

    for coll in collisions {
        resolve(circles, coll.0, coll.1);
    }
}

fn reflect_with_loss(velocity: Vector2, normal: Vector2) -> Vector2 {
    // Coefficient of restitution
    const COR: f64 = 0.95;
    let v_normal = dot(velocity, normal) * normal;
    let v_tangential = velocity - v_normal;
    return v_tangential - COR * v_normal;
}

fn collision_update_with_bounds(c: &mut Circle) {
    const EPS: f64 = 1e-8;

    if (c.position.y - c.radius) - BOUNDS.min.y < EPS {
        c.velocity = reflect_with_loss(c.velocity, Vector2::new(0.0, 1.0));
        if (c.position.y - c.radius) - BOUNDS.min.y < 0.0 {
            c.position.y = c.radius + BOUNDS.min.y;
        }
    }
    if BOUNDS.max.x - (c.position.x + c.radius) < EPS {
        c.velocity = reflect_with_loss(c.velocity, Vector2::new(-1.0, 0.0));
        if BOUNDS.max.x - (c.position.x + c.radius) < 0.0 {
            c.position.x = -c.radius + BOUNDS.max.x;
        }
    }
    if (c.position.x - c.radius) - BOUNDS.min.x < EPS {
        c.velocity = reflect_with_loss(c.velocity, Vector2::new(1.0, 0.0));
        if (c.position.x - c.radius) - BOUNDS.min.x < 0.0 {
            c.position.x = c.radius + BOUNDS.min.x;
        }
    }

    if c.velocity.length() < EPS {
        c.velocity = Vector2::ZERO;
    }
}
