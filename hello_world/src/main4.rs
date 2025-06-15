//Talk about ownership, borrow, and lifetimes is a heart or core concept of rust

fn main(){
    //Ownership Rules:
    //Each value in Rust has a variable that’s its owner.
    //There can only be one owner at a time.
    //When the owner goes out of scope, the value is dropped.
    let s1: String = String::from("Hello World!"); 
    let s2: String = s1; //move ownership from s1 to s2, that's why when we call s1 will be error.
    println!("{}", s2);

    //Borrowing and References:
    //You can borrow values instead of transferring ownership.
    //Use & to reference, and &mut for mutable references.

    let s1: String = String::from("Hello World");
    let length_s1 = calculate_length_of_string(&s1);
    println!("Length of the \"{}\": {}",  s1, length_s1); //Because the calculate_length_of_string does not take ownership of s1, that's why it could be print!

    let mut s1: String = String::from("Hello World");
    change_some_string(&mut s1);
    println!("{}", s1);

}


//function for calculate the length of 
fn calculate_length_of_string(s: &str)->usize{
    s.len()
}

fn change_some_string(s: &mut String){
    let new_string = String::from("!");
    s.push_str(&new_string);
}