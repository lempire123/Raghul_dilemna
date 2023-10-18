use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::{Instant, Duration};

async fn monte_carlo_simulation(count: Arc<Mutex<u32>>, iterations: u32) {
    let rng = ChaChaRng::from_entropy(); // <- Change this
    let mut s: Vec<u32> = (1..=30).collect();
    let mut local_count = 0;

    for _ in 0..iterations {
        s.shuffle(&mut rng.clone()); // Clone the rng for shuffling
        let sum: u32 = s.iter().take(3).sum();
        if sum % 3 == 0 {
            local_count += 1;
        }
    }

    let mut global_count = count.lock().await;
    *global_count += local_count;
}

pub async fn run() -> Duration {
    let start_time = Instant::now();
    let n_simulations = 1_000_000;
    let n_tasks = 8; 
    let simulations_per_task = n_simulations / n_tasks;

    let count = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();
    for _ in 0..n_tasks {
        let count_clone = count.clone();
        let handle = tokio::spawn(monte_carlo_simulation(count_clone, simulations_per_task));
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let probability = *count.lock().await as f64 / n_simulations as f64;

    let estimated_valid_sets = (probability * 4060.0).round() as u32;
    println!("Estimated number of valid sets (asynchronous): {}", estimated_valid_sets);
    start_time.elapsed()
}

