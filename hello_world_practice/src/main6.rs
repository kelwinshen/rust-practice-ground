fn main(){
//  let dangle = get_dangle(); //will get error
let no_dangle = get_no_dangle();
println!("{}", no_dangle);
}

// fn get_dangle ()-> &str{
// let s = String::from("I AM COME HERE FOR DANGLE");
// &s
// }

fn get_no_dangle()->String{
    let s: String = String::from("I AM SAFE");
    s
}