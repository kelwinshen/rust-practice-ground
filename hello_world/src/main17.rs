use std::fs::{self, File};
use std::io::{self, Write};

fn main()->io::Result<()>{
    let mut file = File::create("hello.txt")?;
    file.write_all(b"hello world!")?;
    println!("Successfully wrote hello world!");

    fs::create_dir_all("output")?; //create directory
    println!("Successfully create directory output");


    let mut message_file = File::create("message.txt")?;
    message_file.write_all(b"I am successful write this file some message")?;
    println!("Successful write the message");
    Ok(())
}