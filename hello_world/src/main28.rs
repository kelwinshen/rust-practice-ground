// Trait bounds in Trait:
// When you use trait bounds in a trait definition, you can require that the types involved must implement certain traits. This allows you to ensure that only types with specific behavior can be used in your trait.

//Import some traits from the library
use std::fmt::Display;
use std::cmp::PartialOrd;

//Define some struct example

struct Example;

//Define the traits

trait SortAndDisplay<T: Display + PartialOrd> {
    fn sort_and_display(&self, values:&mut [T]);
}

impl <T:Display+PartialOrd> SortAndDisplay<T> for Example{
    fn sort_and_display(&self, values:&mut [T]){
        values.sort_by(|a,b|a.partial_cmp(b).unwrap());
        for value in values {
            println!("{}", value);
        }
    }
}

fn main(){

    let example = Example;
    let mut my_vector: Vec<u32> = vec![23,5,2,1,3,6,45,23,64,200];
    example.sort_and_display(&mut my_vector);

}