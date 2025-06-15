fn main(){
    //Collections in Rust are used to store multiple values. We'll focus on vectors and hash maps.

    //Vectors are like arrays, but they can grow or shrink dynamically.
    let mut my_first_vector: Vec<i32> = Vec::new();
    
    my_first_vector.push(1);
    my_first_vector.push(2);
    my_first_vector.push(3);
    println!("{:?}", my_first_vector);
    my_first_vector.pop();
    println!("after pop: {:?}", my_first_vector);

    //accessing element in vector

    let second_element_of_vector = my_first_vector[1];
    println!("second element of the vector is {}", second_element_of_vector);

    match my_first_vector.get(1) {
        Some(val) => println!("The value is {}", val),
        None => println!("No value in that index")
    }

}