fn main(){
    let  words = "rust is fun";

    let vec_word: Vec<&str> = words.split_whitespace().collect();

    println!("{:?}", vec_word);


    let length_of_the_vec = vec_word.len();

    let mut words:String = String::new();

    for i in 0..length_of_the_vec {
        words.push_str(vec_word.get(length_of_the_vec-i-1).unwrap());
        words.push_str(" ");
    }
    
    println!("{}", words);
}