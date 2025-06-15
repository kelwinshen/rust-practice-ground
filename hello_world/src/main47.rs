//Closure is anonymous function that can capture some variable from the environment, and its tends to be more flexible than function its self. In function more static. For example the all parameter that already we define have to passed explicitly. However, function can be very useful when we want to do some logic task that reusable. So we can use the function repeatly.

fn sum(x: i32, y: i32)->i32{
    x+y
}

fn main(){
    let x: i32 = 10;
    let add_some = {|n: i32| n + x};
    println!("{}", add_some(2));

    println!("{}", sum(10, 2));
    println!("{}", add_some(2));
}