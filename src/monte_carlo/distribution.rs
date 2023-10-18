use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

pub fn run()  {
    let n_simulations = 1_000_000;  // number of Monte Carlo simulations
    let mut rng = thread_rng();
    let mut s: Vec<u32> = (1..=30).collect();

    // Step 1: Create a vector to store the sums
    let mut sums: Vec<u32> = Vec::new();

    for _ in 0..n_simulations {
        s.shuffle(&mut rng);  // shuffle the set
        let sum: u32 = s.iter().take(3).sum();  // take the first 3 numbers and sum them
        sums.push(sum);
    }

    // Step 2: Create a histogram from the sums
    let mut histogram = HashMap::new();
    for &sum in &sums {
        *histogram.entry(sum).or_insert(0) += 1;
    }

    // Step 3: Sort the keys (sums) and print the histogram
    let mut sorted_sums: Vec<&u32> = histogram.keys().collect();
    sorted_sums.sort();

    let max_count = *histogram.values().max().unwrap_or(&1);
    for &sum in sorted_sums {
        let count = histogram.get(&sum).unwrap();
        let bar_len = (count * 50) / max_count; // Normalizing bar length to a max of 50 characters
        let bar: String = "■".repeat(bar_len as usize);
        println!("{:3} | {:5} | {}", sum, count, bar);
    }
}
