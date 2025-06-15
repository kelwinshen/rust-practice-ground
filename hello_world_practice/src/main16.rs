//Let's Create a Rust Program to Find the Maximum Element in an Array


fn main(){
    let mut input_array: [u32; 5] = [1, 5, 3, 9, 2];
    //n log n complexity
    input_array.sort();

    println!("The maximum element is: {}",  input_array[&input_array.len()-1]);




}