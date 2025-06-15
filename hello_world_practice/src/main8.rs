use std::io::{self, Read};
use std::fs::{ File};

fn main()->io::Result<()>{

    let hello_file = File::open("src/hello.txt");


    match hello_file {
        Ok(mut content) => {
            let mut hello_file_content = String::new();
            content.read_to_string(&mut hello_file_content)?;
            println!("The content is: {}", hello_file_content)},
        Err(error) => {
            println!("Hello file cannot be opened!, the error: {}", error)}
    }


    let settings_file = File::open("src/config/settings.txt");

    match settings_file{
        Ok(mut settings)=>{
            let mut settings_string: String = String::new();
               settings.read_to_string(&mut settings_string)?;
            println!("The settings: {}", settings_string);
        },
        Err(error)=>{
            println!("The settings not found! Some errors: {}", error);
        }
    }
    Ok(())
}