use std::collections::HashMap;

// Key:
// Used to uniquely identify data.
// It can be of any type that implements the Eq and Hash traits (such as integers, strings, etc.).

// Value:
// The data associated with the key.
// Each key can only have one value associated with it at a time. If you try to insert a new value with an existing key, the old value is overwritten.

// How It Works:
// The hash function is used to calculate a hash for the key, determining its location in the collection.
// This ensures that insertion, retrieval, updating, and deletion operations are very efficient, with an average complexity of O(1).

// Collisions:
// If two keys hash to the same bucket (collision), Rust uses a chaining method to handle the collision, by storing multiple values in the same bucket (typically using a linked list).


fn main(){
    //Hash maps store key-value pairs.
    let mut my_first_hashmap: HashMap<i32, i32> = HashMap::new();

    let mut i = 0;
    while i < 10 {
        my_first_hashmap.insert(i, i + 1);
        i += 1;
    }

    println!("{:?}", my_first_hashmap);

    let target_key: i32 = 5;

    match my_first_hashmap.get(&target_key) {
        Some(&value) => println!("The value: {}", value),
        None => println!("Not found that value")
    };

    //remove some key pair by this method
    my_first_hashmap.remove(&1);

    println!("{:?}", my_first_hashmap);

    //check certain key is exist or not in hashmap

    if !my_first_hashmap.contains_key(&1) {
        println!("The key and value for that one is already remove ");
    }
    
   
}


