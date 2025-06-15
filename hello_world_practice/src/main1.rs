use std::io;

fn main(){
    println!("name:");
    let mut name: String = String::new();
   io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("age:");
    let mut age: String = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = match age.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please input a correct age!");
            return;
        }
    };

    println!("gpa:");
    let mut gpa: String = String::new();
    io::stdin().read_line(&mut gpa).expect("Failed to read line");
    let gpa: f64 = match gpa.trim().parse(){
        Ok(float) => float,
        Err(_)=>{
            println!("Please input a correct gpa!");
            return;
        }
    };

    println!("----------------------");
  

    let tup: (String, u32, f64) = (name, age, gpa);

    println!("Select option with the tuple!");
    println!("1: Print the entire tuple.\n2: Print a summary of the values (name in uppercase, age increased by 5, score rounded to 2 decimal places).\n3: Destructure the tuple and print each value on a separate line.");
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
   
    let choice: u32 = match choice.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("There is no that operation");
            return;
        }
    };

    match choice{
        1 => {
            println!("{:?}", &tup);
        } ,
        2 => {
            print!("Name: {}", &tup.0.to_uppercase());
            println!("Age: {}", &tup.1 + 5);
            println!("Gpa: {:.2}", &tup.2);
        },
        3 =>{
           let (a,b,c) = tup;
           print!("{}", a);
           println!("{}", b);
           println!("{}", c);
        },
        _=>{
            println!("Something wrong with the choice!");
        }
    };

}