//Let's handle some error with option

fn do_something(value: u32)->Option<u32>{
    if *&value <= 0 {
        return Some(value);
    } else{
        return None;
    }
}
fn main(){
    println!("{:?}", do_something(0));
}