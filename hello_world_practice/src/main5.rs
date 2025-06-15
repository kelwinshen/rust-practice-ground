use std::collections::HashMap;

fn main() {
    let mut transactions_value: [u32; 13] = [34, 7, 23, 32, 5, 62, 7, 23, 5, 5, 34, 62, 23];
    println!("transaction before sort{:?}", transactions_value);

    //let's sort the transaction value first. The sort process complexity basically is n log (n)
    transactions_value.sort();
    println!("transaction after sort: {:?}", transactions_value);

    //next search for a specific transaction index of certain value. The process is call binary search with complexity is log(n)
    let target_transactions: u32 = 32;

   match transactions_value.binary_search(&target_transactions){ 
    Ok(num) => {
        println!("the index of {} is {}", &target_transactions,  num);
    }, 
    Err(_) => {
        println!("That transaction not found!");
    }
   };

   //next we want to know the distribution of frequency of each transaction, in this part we will use collection hashmap to determine each frequency occurs of the transactions.
   //the complexity is O(n) 

   let mut frequency_map = HashMap::new();

   for &transactions in &transactions_value {
        let counter = frequency_map.entry(transactions).or_insert(0); //it's mean we get into the value by the key if its already exist, if it don't yet have that key, we insert the key and the value is 0
        *counter += 1; //dereference 
   }

   println!("The frequency map: {:?}", frequency_map);

   //next if we want to find the median if we already sorted the list is just O(1), but before that define the function for find the median


   let median = find_the_median(&transactions_value);
   println!("the median is {}", median);

}


fn find_the_median (sortedlist: &[u32; 13] )->f64{
    let length_of_the_list = sortedlist.len();
    if length_of_the_list % 2 == 0 {
        let median = (sortedlist[length_of_the_list/2] as f64 +  sortedlist[length_of_the_list/2 - 1] as f64 )/2.0;
        median
    } else  {
        let median = sortedlist[(length_of_the_list-1)/2] as f64;
        median
    }
}