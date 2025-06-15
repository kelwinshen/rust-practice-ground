use std::fs::File;
use std::io::{self,Read};


fn read_file()->Result<String, io::Error>{
    let mut content: String = String::new();
    let mut open_file = File::open("src/test.txt")?;
    open_file.read_to_string(&mut content)?;
    Ok(content)
}

fn main(){
    match read_file(){
        Ok(content) => println!("the content is {}", content),
        Err(error) => println!("The content is unavailable, due to: {}", error)
    }
}