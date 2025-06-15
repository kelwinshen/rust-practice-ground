// Async Programming in Rust
// Rust also has asynchronous programming support via async and await, allowing you to write non-blocking, concurrent code. It’s useful when working with tasks like I/O operations, which involve waiting for data (e.g., reading from a file or making HTTP requests).

use std::time::Duration;
use tokio::time::sleep; // `tokio` runtime is required for this example

async fn do_work() {
    println!("Starting work...");
    sleep(Duration::from_secs(2)).await; // Simulate async task
    println!("Work done!");
}

#[tokio::main]
async fn main() {
    let task1 = do_work();
    let task2 = do_work();
    let task3 = do_work();

    tokio::join!(task1, task2, task3); // Run both tasks concurrently
}
