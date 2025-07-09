use std::time::Instant;

pub fn run_brute_force(n: i64) {
    let mut hit_count = 0;
    let start = Instant::now();
    for _ in 0..n {
        let x = rand::random_range(0.0..1.0);
        let y = rand::random_range(0.0..1.0);
        let l = x * x + y * y;
        if l <= 1.0 {
            hit_count += 1;
        }
    }

    let elapsed = start.elapsed();
    println!("Elapsed {:?}", elapsed);

    let pi_estimate = (4.0 * hit_count as f64) / n as f64;
    println!("{:?}", pi_estimate);
}

pub fn run_stratified(sqrt_n: i64) {
    let mut hit_count = 0;
    let start = Instant::now();

    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let x = 2.0 * (i as f64 + rand::random::<f64>()) / (sqrt_n as f64) - 1.0;
            let y = 2.0 * (j as f64 + rand::random::<f64>()) / (sqrt_n as f64) - 1.0;
            let l = x * x + y * y;
            if l <= 1.0 {
                hit_count += 1;
            }
        }
    }

    let total_count = sqrt_n * sqrt_n;

    let elapsed = start.elapsed();
    println!("Elapsed {:?}", elapsed);

    let pi_estimate = (4.0 * hit_count as f64) / total_count as f64;
    println!("{:?}", pi_estimate);
}
