use std::fs::File;
use std::io::{Read, Error};

//Let's try some recoverable error using propagating Errors
fn read_file(path: &str)->Result<String, Error>{
    let mut result = File::open(path)?;
    let mut content: String = String::new();
    result.read_to_string(&mut content)?;
    Ok(content)
}


fn main(){
    let path_to_my_file = "sr/test.txt";
    match read_file(path_to_my_file) {
        Ok(content) => {
            println!("The content is: {}", &content);
        },
        Err(e) => {
            println!("Something wrong when read the file, due to: {}", e);
        }
    }

}