fn find_element_in_vector(vecs: &Vec<i32> , index: usize)->Result<i32, &'static str>{
    vecs.get(index).ok_or("out of scope").copied()
}

fn main(){
    let my_vector: Vec<i32> = vec![1,11,111,1111,11111];
    match find_element_in_vector(&my_vector, 1) {
        Ok(number)=>println!("the number is {}", number),
        Err(error)=>println!("Error: {}", error)
    }
}