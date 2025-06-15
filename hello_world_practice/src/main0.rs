use std::io; 

fn main() {
    let tup: (i32, i32, i32) = (5, 10,15);
    println!("The tuple is: {:?}", tup);
    println!("1. Sum of values\n2. Product of values\n3. Print each value separately");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice, please enter 1,2, or 3");
            return;
        }
    };

    match choice{
        1 => {
            let result: i32 = tup.0 + tup.1 + tup.2;
            println!("The result is: {}", result);
        },
        2 => {
            let result: i32 = tup.0 * tup.1 * tup.2;
            println!("The result is: {}", result);
        },
        3 => {let (a,b,c) =  tup;
            println!("The value of a: {}", a);
            println!("The value of b: {}", b);
            println!("The value of c: {}", c);
        },
        _ => {
            println!("Please make sure the input 1,2, or 3 ");
        }
    };
}
