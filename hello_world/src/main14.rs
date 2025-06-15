use std::fs::File;

fn main(){
    //The Result enum is used to handle recoverable errors.
    let file_result = File::open("hello.txt");

    match file_result {
        Ok(file) => {
            println!("File open successfully: {:?}", file);
        },
        Err(error) => {
            println!("File failed to open! The error: {:?}", error);
        }
    };

    //Unrecoverable Errors 

    let mut my_vector: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < 100 {
        my_vector.push(i);
        i = i + 1;
    }

   
    println!("my vector: {:?}", &my_vector);

    let not_found_value: &i32 = &my_vector[101];
    

}