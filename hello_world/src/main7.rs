fn main(){
    let s1: String = String::from("ABCD");
    let s2: String = String::from("xyz");

    let result: &str = pick_longest_string(&s1, &s2);
    println!("The longest string is {}", result);

}

//Lifetimes are how Rust ensures that references are always valid. Lifetimes prevent dangling references by enforcing rules about how long a reference can be used.
fn pick_longest_string<'a> (x: &'a str, y: &'a str)-> &'a str{
    if x.len() > y.len() {
        x
    } else  {
        y
    } 
}




