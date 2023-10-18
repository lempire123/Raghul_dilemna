use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{Instant, Duration};

pub fn run() -> Duration {
    let start_time = Instant::now();

    let n_simulations = 1_000_000;  // number of Monte Carlo simulations
    let mut count = 0;  // count of sets where sum is divisible by 3

    let mut rng = thread_rng();
    let mut s: Vec<u32> = (1..=30).collect();

    for _ in 0..n_simulations {
        s.shuffle(&mut rng);  // shuffle the set
        let sum: u32 = s.iter().take(3).sum();  // take the first 3 numbers and sum them
        if sum % 3 == 0 {
            count += 1;
        }
    }
    let probability = count as f64 / n_simulations as f64;
    // Since there are (30 choose 3) = 4060 possible sets, we estimate the number of valid sets as:
    let estimated_valid_sets = (probability * 4060.0).round() as u32;
    println!("Estimated number of valid sets (synchronous): {}", estimated_valid_sets);
    start_time.elapsed()
}