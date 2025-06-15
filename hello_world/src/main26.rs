// What Are Trait Bounds? 
// Trait bounds specify that a generic type must implement certain traits (behaviors) in order to be used in a function, struct, or trait. This ensures that the operations or methods you're using in the code will work for any type that satisfies the bound.

// Why Use Trait Bounds?
// Safety: Ensures that your generic type has the required functionality (e.g., comparison, printing, etc.).
// Flexibility: You can write generic code that works with any type, as long as the type meets certain requirements.


//Trait Bounds in function, make sure that type in that functions working with certain behavior because the operation is needed as that in that function

use std::fmt::Display;
use std::ops::Add;

fn printout<T: std::fmt::Display >(item: T){
    println!("{}", item);
}

fn addition_printout<T:  Display + Add<Output= T> + Copy>(a:T, b:T){
    let result = a+b;
    println!("{} + {} = {}", a, b,result);
}

fn main(){
    let test = 12345;
    printout(test);

    addition_printout(1,1);
}