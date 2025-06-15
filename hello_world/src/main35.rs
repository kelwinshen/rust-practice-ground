// collect is a method to make a result of iter collected back to a collection

fn modify_list(mut values:  Vec<i32>){
    values =  values.iter().map(|value| value+1).collect();
    println!("{:?}", values);
}

fn main(){
let before_modify_list: Vec<i32> = vec![1,11,111,1111,11111,111111];

modify_list(before_modify_list);


}