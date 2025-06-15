fn main(){
    //Rust automatically “moves” ownership when assigning variables or passing them to functions.

    let s1: String = String::from("Hello World!");
    let s2 = s1; //Move ownership from s1 to s2 , if dont want to let s2 take the ownership, could be use reference or clone
    println!("{}", s2);


    let int1: u32 = 15;
    let int2 = int1; //don't take ownership, but copy.  
    //Types like integers (i32, u8), booleans (bool), and characters (char) are Copy types.

    println!("{} {}", int1, int2);

    //function also take ownership of certain variable if the argument pass to the function is take the ownership, for example:
    let s3 =  String::from("I am ready to taken");
    take_ownership(s3);
   //  println!("{}", s0); this will make error because s0 already move the ownership to the function argument



   let s4 = String::from("Let's calculate how long I am");
   let length: usize = just_borrow_to_calculate_length(&s4);
   println!("The length of {}, is {}", s4, length);

}


fn take_ownership(s: String){
    println!("{}",s);
}

fn just_borrow_to_calculate_length(s: &String)->usize{
    s.len()
}

