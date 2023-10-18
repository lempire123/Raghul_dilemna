mod monte_carlo;

use monte_carlo::asynchronous;
use monte_carlo::synchronous;

#[tokio::main]
async fn main() {
    // Running and benchmarking the concurrent version
    let concurrent_duration = asynchronous::run().await;

    // Running and benchmarking the sequential version
    let sequential_duration = synchronous::run();

    // Print the results
    println!(
        "Concurrent took {:?} and Sequential took {:?}",
        concurrent_duration, sequential_duration
    );
}
