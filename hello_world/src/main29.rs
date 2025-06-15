use std::cmp::PartialOrd;
use std::fmt::Display;

//Define some traits 

trait  NumberComparison<T: PartialOrd + Display> {
    fn compare_number(&self, a:T, b:T);
}

impl NumberComparison<i32> for i32 {
    fn compare_number(&self, a:i32 , b:i32){
        if a < b {
            println!("{} is less than {}", a, b);
        } else if a > b{
            println!("{} is greater than {}", a, b);
        } else {
            println!("{} is equal to {}", a, b);
        }
    }
}



fn main(){
    let number: i32 = 0;
    number.compare_number(-1,1);
}