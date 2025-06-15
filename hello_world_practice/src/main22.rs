

fn main(){
    let strings = "Rust makes programming more fun";
    let vec_strings: Vec<&str> = strings.split_whitespace().collect();
    println!("{:?}", vec_strings);

    let mut longest_string = "";
    let mut longest = 0;
    for string in vec_strings{
       if string.len() > longest {
        longest = string.len();
        longest_string = string;
       }
    }

    println!("The longest string is: {}", longest_string);
}