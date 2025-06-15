//Shared State Between Threads
//Rust prevents data races by enforcing ownership rules in concurrent contexts. If multiple threads need access to the same data, you’ll need to use Arc (Atomic Reference Counting) and Mutex to manage shared state safely.



use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));  // Shared counter wrapped in Arc<Mutex>
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);  // Clone the Arc for each thread

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // Lock the mutex before modifying
            *num += 1;  // Increment the counter
        });

        handles.push(handle);  // Store the thread handle
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Final Counter Value: {}", *counter.lock().unwrap());
}
