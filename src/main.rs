use rand::seq::SliceRandom;  // for shuffle
use rand::thread_rng;

fn main() {
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
    // println!("Count of sets where the sum is divisible by 3: {}", count);
    let probability = count as f64 / n_simulations as f64;
    println!("Probability that the sum of the first 3 numbers is divisible by 3: {}", probability);
    // Since there are (30 choose 3) = 4060 possible sets, we estimate the number of valid sets as:
    let estimated_valid_sets = (probability * 4060.0).round() as u32;

    println!("Estimated number of sets where the sum is divisible by 3: {}", estimated_valid_sets);
}

