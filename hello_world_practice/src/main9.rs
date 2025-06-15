use std::io::{self, Read};
use std::fs::{File};


fn read_file(path: &str)->io::Result<String>{
let mut file_hello = File::open(path)?;
let mut content:String = String::new();
file_hello.read_to_string(&mut content)?;
Ok(content)
}

fn main()->io::Result<()>{
    let helloworld_path: String = String::from("src/hello.txt");
    let helloworld_content = read_file(&helloworld_path)?;
    println!("The content of path: {} is {}", helloworld_path, helloworld_content);
    Ok(())
}