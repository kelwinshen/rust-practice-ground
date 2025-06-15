//enumerate is use for give index to each item that we access

fn find_number_in_vector(values: Vec<i32>)->Option<usize>{
    for (index, value) in values.iter().enumerate(){
       if value == &111 as &i32 {
        return Some(index);
       } 
    
    }
    None
}

fn main(){
    let my_vector: Vec<i32> = vec![1,11,111,1111,11111];
    println!("{:?}", find_number_in_vector(my_vector));
}