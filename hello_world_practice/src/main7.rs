use std::collections::HashMap;

fn main(){


    let mut sentence: String = String::from("Rust is fast and memory efficient with no runtime or garbage collector. Rust allows you to build reliable and efficient software.");
    //convert the sentence to lowercase.
    sentence = sentence.to_lowercase();
    println!("{}", &sentence);

    //remove all punctuation by replace 
    sentence = sentence.replace(".", "");
    println!("{}", &sentence);


    //make the sentence to vector
    let sentence_vector: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", sentence_vector);

    //create a hashmap
    let mut frequency_map = HashMap::new();

    for &word in &sentence_vector{
        let counter = frequency_map.entry(word).or_insert(0);
        *counter += 1;
    }

    println!("{:?}", frequency_map);


    //find the word that has highest frequency

    let mut highest_frequency_word: String = String::new();
    let mut highest_frequency = 0;

    for &word in &sentence_vector{
       if *frequency_map.get(&word).unwrap() > highest_frequency {
        highest_frequency_word = String::from(word);
        highest_frequency = *frequency_map.get(&word).unwrap();
       }
    }

    println!("The highest frequency word is {}, with the frequency is {}",highest_frequency_word,  highest_frequency);


}