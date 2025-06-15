
//Compound Types: Rust has tuples and arrays
fn main(){
    println!("Start from tuple");
    let tup: (i32, f64, u8) = (-1, 3.14, 2);
    println!("{:?}", tup);
    println!("Let's do some destructuring");
    let (x,y,z) =  tup;
    println!("this is x: {}", x);
    println!("this is y: {}", y);
    println!("this is z: {}", z);
    println!("Start from array");
    let arr: [u32; 4] = [1,2,3,4]; //fixed size, already know when compiled.
    println!("{:?}", arr);
}