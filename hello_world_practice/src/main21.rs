//filter words by length, from the highest length to lowest length


fn main(){

    let words = "Rust is amazing and fast";

    let mut highest_length = 0;
    
    let vec_word: Vec<&str> = words.split_whitespace().collect();

    for word in &vec_word{
        if word.len() > highest_length{
            highest_length = word.len();
        }
    }

    let mut vec_word_filtered: Vec<&str> = vec![];

    for i in 1..=highest_length {
        let vec_word_sub_filtered: Vec<&str> = words.split_whitespace().filter(|word| word.len() == i).collect();
        vec_word_filtered.extend(vec_word_sub_filtered);
    }


    println!("{:?}", vec_word_filtered);



}


