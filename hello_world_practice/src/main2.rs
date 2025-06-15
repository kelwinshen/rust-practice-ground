use std::io;


fn main(){
    //Node is running fine

    let mut input_string: String = String::new();
    io::stdin().read_line(&mut input_string).expect("Please input the string correctly!");

    println!("{}", &input_string);


    let array_string: Vec<&str> = input_string.split_whitespace().collect(); //if we want let compiler to filled the type of Vec, its possible, just let write Vec<_>
    //Use .collect() when you need to store and access all elements later. 
    // Use just the iterator if you only need to process elements once in a loop.
    
    println!("{:?}", array_string);


    let mut my_string: String = String::new();

    for x in array_string {
        my_string.push_str(x); //push_str method for literal string
        my_string.push(' '); //push method for char
    }

    println!("{}", my_string);

 
    

}