// Channels for Thread Communication
// Rust has built-in channels that allow threads to communicate safely by sending messages between them.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); // Create a channel

    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread!"];
        for message in messages {
            tx.send(message).unwrap(); // Send messages to the receiver
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received); // Receive and print messages
    }
}

