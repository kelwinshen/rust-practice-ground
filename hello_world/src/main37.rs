//Recoverable Error: Result<T, E>

//Most errors in Rust are recoverable, and they are handled using the Result enum, which has two variants

//Ok(T): Represents a successful operation.
//Err(E): Represents an error.

use std::fs::File;
use std::io::Read;

fn main(){
    let open_file = File::open("src/test.txt");
    match open_file{
        Ok(mut file) => {
            let mut content: String = String::new();
            let _ = file.read_to_string(&mut content);
            println!("The file content is: {}", content);
        },
        Err(error)=> {
            println!("Something wrong when open the file, due to: {}", error);
        }
    }
}