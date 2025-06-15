
//Use closure as argument in a function

fn closure_function<F>(num: u32, f:F)-> u32 where F: Fn(u32)->u32, {
    f(num)
}

fn main(){
    let x: u32 = 50;
    let closure = |a| x + a;

    println!("{}", closure_function(10, closure));

}