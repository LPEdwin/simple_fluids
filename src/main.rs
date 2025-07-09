mod estimate_pi;
mod vec3;

fn main() {
    estimate_pi::run_brute_force(1_000_000_00);
    estimate_pi::run_stratified(1_000_0);
}
