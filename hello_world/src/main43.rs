// Concurrency is about executing multiple tasks at the same time, and Rust’s approach to concurrency is unique because it ensures memory safety while preventing data races. In this section, we will focus on Rust’s tools for writing concurrent programs, such as threads and the async/await syntax.


//1. Thread in Rust
//Rust provides threads for concurrency via the std::thread module. You can create multiple threads to perform tasks in parallel.

use std::thread;
use std::time::Duration;

fn main(){
    let handle_thread = thread::spawn(||{
        for i in 0..20{
            println!("thread: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 0..50{
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(300));
    };

    handle_thread.join().unwrap();
}