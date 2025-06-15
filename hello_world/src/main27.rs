//Try to use generic trait for struct
use std::fmt::Display;

struct Pair<T: Display + PartialOrd> {
    x: T,
    y:T
}

impl <T: Display + PartialOrd>  Pair<T>{
    fn compare(&self){
       if &self.x > &self.y {
        println!("{} is greater than {}", &self.x, &self.y);
       } else if &self.x < &self.y {
        println!("{} is less than {}", &self.x, &self.y);
       } else {
        println!("{} is equal to {}", &self.x, &self.y);
       }
    }
}

fn main(){
    let pair_of_number = Pair{
        x: 10,
        y: 10
    };

   pair_of_number.compare();
}