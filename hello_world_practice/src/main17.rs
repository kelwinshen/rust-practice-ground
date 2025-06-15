//capitalize the first letter
fn main(){
    let name = "alice";
    let capitalized = format! ("{}{}",  name.chars().next().unwrap().to_uppercase(), &name[1..5]);
    
    println!("{}", capitalized);
}