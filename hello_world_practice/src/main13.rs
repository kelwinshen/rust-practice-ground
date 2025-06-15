use std::thread;
use std::sync::{Arc, Mutex};

fn main(){
    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let thread = thread::spawn(move||{
           let mut num = counter.lock().unwrap();
            if i%2 == 0 {
                *num +=1;
            } else {
                *num +=2;
            }
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("The counter value is: {}", counter.lock().unwrap());
}