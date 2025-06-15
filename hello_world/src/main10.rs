
//Figure out how many fibbonaci number up to 20 elements has a last digit 1

// 0,1,1,2,3,5,8,13,21, ...

//0
//0+1 = 1
//1+1 = 2
//1+2 = 3
//2+3 = 5

struct FibbonaciNumber{
    current_number:  u128,
    next_number: u128
}



fn main(){

 let mut list_of_fibbonaci = Vec::new();

let mut my_fibbonaci = FibbonaciNumber{
    current_number: 0,
    next_number: 1
};


let mut counter = 0;

//infinite loop until break
 loop {
    list_of_fibbonaci.push(my_fibbonaci.current_number.clone());
    let temporary_current_number =  my_fibbonaci.current_number.clone();
    my_fibbonaci.current_number =  my_fibbonaci.next_number.clone();
     my_fibbonaci.next_number = temporary_current_number+ my_fibbonaci.next_number;
    counter = counter + 1;
    if counter == 100 {
        break;
    }
}

list_of_fibbonaci.reverse(); //to reverse some vector 


println!("{:?}", list_of_fibbonaci);


let mut amount = 0;

//for loop
    for x in &list_of_fibbonaci{
        if x%10 == 1 {
            amount = amount + 1;
        }
    }


println!("Total of number with last digit 1 is: {}", amount);


//find the index of the list the first time has last digit 8 using while loop

let mut i = 0;
let mut sum = 0;

while i != 99 {
    i = i + 1;
   sum = sum + list_of_fibbonaci[i];
}

println!("Sum of 100 first fibbonaci is: {}", sum);



    

  

    
}