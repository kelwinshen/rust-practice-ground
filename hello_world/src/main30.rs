//Lifetimes: For anotates a variable how long does it live so when we call it don't get dangling if it already dropped

fn get_longest_string<'a>(words:  Vec<&'a str>)-> &'a str {
    let mut longest_string = "";
    for word in words {
        if word.len() > longest_string.len(){
            longest_string = word;
        }
    }
    longest_string
}

fn main(){
    let vocabulary: Vec<&str> = vec!["Hello", "World", "Rust", "Rustacean", "Supra Oracles"];
     println!("The longest string is: {}", get_longest_string(vocabulary));   
}