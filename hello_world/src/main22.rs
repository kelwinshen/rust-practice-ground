//Generics allow you to write code that can operate on different types without needing to duplicate code for each type. This is done by introducing type parameters into your functions, structs, enums, and traits.

//Generic Function

fn largest<T: PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];

    for element in list {
        if element > largest {
            largest = element;
        }
    }
    largest
}

fn smallest<T: PartialOrd>(list: &Vec<T>) -> &T {
    let mut smallest = &list[0];
    for element in list.iter() {
        if element < smallest {
            smallest = element;
        }
    }
    smallest
}

fn main() {
    let transaction_value: Vec<u64> = vec![24, 124, 53, 4, 12, 52, 424, 21, 4, 124, 5, 6];
    println!("The largest number is: {}", largest(&transaction_value));
    println!("The smallest number is: {}", smallest(&transaction_value));

    let transaction_value: Vec<f64> = vec![1.2, 2.52, 0.12, 3.14, 4.2];
    println!("The largest number is: {}", largest(&transaction_value));
    println!("The smallest number is: {}", smallest(&transaction_value));

    let char_list: Vec<char> = vec!['a', 'z', 'c', 'w'];
    println!("The largest characters is: {}", largest(&char_list));
    println!("The smallest characters is: {}", smallest(&char_list));

}
