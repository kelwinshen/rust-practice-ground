//By default, variables in Rust are immutable (meaning you cannot change their value once assigned).
fn main(){
    let x = "5";
    println!("In number, five is: {}", x);
    //If you need to change the value, make it mutable with mut
    let mut y = "5";
    println!("y is mutable, the valaue is: {}", y);
    y="6";
    println!("after change the value, the value now is {}", y);
}