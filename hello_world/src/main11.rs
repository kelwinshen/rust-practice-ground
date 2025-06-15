use std::collections::HashMap;

//The logic way to think, when we do some sorting it's means n * log (n), log (n), n, 1 


fn main(){
    let mut arr: [u32; 13] = [34, 7, 23, 32, 5, 62, 7, 23, 5, 5, 34, 62, 23];
    arr.sort(); // The complexity is O(n log n) , means relatively efficient for sort a large list
    println!("{:?}", arr);

    let target_of_search: u32 = 34;

    match arr.binary_search(&target_of_search) { //The complexity is O(log n)
        Ok(num) => println!("The index of the target of search: {}", num),
        Err(_) => println!("Not found that number!")
    }; 

    //(O(n) complexity)
    let mut frequency_map = HashMap::new();
    for &value in &arr {
        let counter = frequency_map.entry(value).or_insert(0);
        *counter += 1;

    }

    println!("Transaction Frequencies: {:?}", frequency_map);

    
    //Finding the Median Value - O(1) after Sorting
    let median = find_median(&arr);
    println!("The median is: {}", median);

}

fn find_median(sorted_arr: &[u32; 13]) -> f64{
    let len = sorted_arr.len();
    if len%2 == 0 {
        let mid1 = sorted_arr[len/2 - 1];
        let mid2 = sorted_arr[len/2];
        let median = (mid1 as f64 +mid2 as f64)/2.0;
        median
    } else {
        let median = sorted_arr[len/2] as f64;
        median
    }

}