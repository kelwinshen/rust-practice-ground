use std::fs::File;
//unwrap() and expect()
//Sometimes, when you are certain that a function won’t fail, you can use unwrap() or expect() to get the value from a Result<T, E> or Option<T>.
fn main(){
    // let file = File::open("hello.txt").unwrap(); // Panics if there’s an error
    let file = File::open("ahello.txt").expect("Failed to open the file"); // Panics with custom error message
}