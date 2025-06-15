//iter -> use for access item in collection


fn main(){
let array: Vec<i32> = vec![1,11,111,1111,11111];

let iter = array.iter();

println!("{:?}", iter.next());
println!("{:?}", iter.next());
println!("{:?}", iter.next());
}