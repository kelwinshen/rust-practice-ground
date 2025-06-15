fn main(){
    let s1: String = String::from("This is first string");
    let s2: String = String::from("This is second string");
    take_ownership(s1);
    let result: usize = calculate_the_length_of_string(&s2);

    println!("The length of {} is {}", s2, result);
}

fn take_ownership(x: String){
    println!("{}", x);
}

fn calculate_the_length_of_string(x: &str)->usize{
    x.len()
}