use std::collections::HashMap;

fn main(){

    let data: Vec<u32> =  vec![1, 2, 3, 3, 4, 2, 3, 5, 2, 2];

    let mut distribution: HashMap<u32, u32> = HashMap::new();

    for num in data{
        let freq = distribution.entry(num).or_insert(0);
        *freq += 1;
    }

    let mut highest_freq: u32 = 0;


    for (_, freq) in &distribution{
        if freq > &highest_freq {
            highest_freq = *freq;
        }
    }

    let mut highest_freq_number_vec : Vec<u32> = vec![];


    for (num, freq) in &distribution{
        if freq == &highest_freq {
            highest_freq_number_vec.push(*num);
        }
    }

    println!("The highest freq number are: {:?}", highest_freq_number_vec);

}