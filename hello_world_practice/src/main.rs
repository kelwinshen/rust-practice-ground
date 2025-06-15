use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();  // Create a channel for sending results

    let ranges = vec![
        (1, 3),   // Thread 1 will sum numbers from 1 to 3
        (4, 6),   // Thread 2 will sum numbers from 4 to 6
        (7, 10),  // Thread 3 will sum numbers from 7 to 10
    ];

    for (start, end) in ranges {
        let tx = tx.clone();  // Clone the transmitter for each thread

        thread::spawn(move || {
            let sum: i32 = (start..=end).sum();  // Sum the range
            tx.send(sum).unwrap();  // Send the result back to the main thread
        });
    }

    let mut total_sum = 0;
    
    // Receive partial sums from the threads and compute the total sum
    for _ in 0..3 {
        total_sum += rx.recv().unwrap();  // Receive each partial sum
    }

    println!("Total Sum: {}", total_sum);
}
