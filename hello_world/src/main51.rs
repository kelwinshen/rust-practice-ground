use std::cmp::PartialOrd;
use std::fmt::Display;

struct Pair<T: PartialOrd + Display>{
    x: T,
    y: T
}


impl <T: PartialOrd + Display> Pair<T> {
    fn min(&self){
       if self.x < self.y {
      println!("The minimum value is: {}", self.x);
       } else {
        println!("The minimum value is: {}", self.y);
       }
    }
}

fn main(){
    let first_pair = Pair{
        x: 10 as u32,
        y: 20 as u32
    };

    first_pair.min();

    let second_pair = Pair{
        x: 3.14 as f64,
        y: 2.71 as f64
    };

    second_pair.min()

    // min(10, 20) or min(3.14, 2.71)
}





