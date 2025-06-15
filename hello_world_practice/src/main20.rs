fn main(){
    let strings = "hello rustaceans";

    let vector_string: Vec<_> = strings.split_whitespace().collect();

    println!("{:?}", vector_string);

    let mut strings = String::new();

    for (index, string) in vector_string.iter().enumerate(){
        let new_string = format! ("{}{}",  string.chars().next().unwrap().to_uppercase(), &string[1..string.len()]);
        if index == string.len() - 1 {
            strings.push_str(&new_string);
        } else {
            strings.push_str(&new_string);
            strings.push_str(" ");
        }
        
    }
   
    println!("{}", strings);

}


    