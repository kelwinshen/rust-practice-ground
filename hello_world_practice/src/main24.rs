use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's Guess The Number");
	let secret_number = rand::thread_rng().gen_range(0..=100);
    loop {
        println!("Input the number your guess");
        let mut guess = String::new();
        let _ = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse(){
	        Ok(num)=>num,
	        Err(_)=> continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
