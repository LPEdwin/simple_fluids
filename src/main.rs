use crate::vec2::Vec2;

mod vec2;

fn main() {
    let n = 1000;
    let c = 0.4;
    let mut in_bounds = 0;
    for _ in 0..n {
        let v = Vec2::random_in_square(0.0..1.0);
        if bound(v.x, v.y) <= c {
            in_bounds += 1;
        }
    }
    let p = in_bounds as f64 / n as f64;
    let correct_value = 2.0 * c - c * c;

    println!(
        "Probability is {} with error {} and correct value{}",
        p,
        (correct_value - p).abs(),
        correct_value
    );
}

fn bound(a: f64, b: f64) -> f64 {
    (a - b).abs()
}
