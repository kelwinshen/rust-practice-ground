use std::process::Command;


fn main(){

    Command::new("echo").arg("Hello from process").spawn().expect("Something wrong! not working");
    
}