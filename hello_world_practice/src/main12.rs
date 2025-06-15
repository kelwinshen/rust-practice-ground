use std::thread;

fn main(){
    let vec_string = vec![
        String::from("Hello!"),
        String::from("I am"),
        String::from("Rust"),
        String::from("Programmer")
    ];

    let mut threads = vec![];

    for element in vec_string {
        let thread = thread::spawn(move|| {
            println!("{}", element);
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }


   


}