//input: hello
//outpit: olleh

use std::io;


fn reverse_string(input: String)->String{
    let mut vector_chars: Vec<char> = input.chars().collect();
    vector_chars.reverse();
    vector_chars.iter().collect()
}

fn main(){
    let mut input_string: String = String::new();
    io::stdin().read_line(&mut input_string).expect("Something wrong with the input!");
    println!("The result of reverse: {}", reverse_string(input_string));
}


