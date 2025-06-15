use std::fs::File;

fn main(){

    let open_file_result_not_panicking = File::open("never_found_file.txt").expect("file not found");

    //Using unwrap() and expect()
    let open_file_result_panick = File::open("never_found_file.txt").unwrap(); //make panicking!


}