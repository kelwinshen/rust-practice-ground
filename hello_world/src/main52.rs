fn first_word(s: &str)->String{
    let sentences = String::from(s);
    let vec_word: Vec<_> = sentences.split_whitespace().collect();
    let result_string = vec_word.get(0).unwrap().to_string();
    result_string

}

fn main(){
    println!("{}", first_word("Rust is amazing!"));
}