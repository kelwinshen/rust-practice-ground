//Talk about how to handle error in Rust
//There will be Recoverable Error and Unrecoverable Error


//Unrecoverable Error
//Sometimes, an error is so severe that the program cannot continue. For these cases, Rust has the panic! macro, which stops the program and prints an error message.

fn main(){
    //Unrecoverable Error because we have handle something wrong

    let my_vector: Vec<i32> = vec![1,2,3,4,5,6];

    println!("{}", &my_vector[6]);


    //Direct Unrecoverable Error
    panic!("Something went wrong!"); //This is we use when we need to directly to panic, tell the program stop immediately. This is tends to use when we develop program, so we could easy spot out which error that we must be handled

}