//Lifetime Elision Rules
// Rust tries to infer lifetimes to reduce the need for explicit annotations. In many cases, Rust applies lifetime elision rules that allow you to omit lifetime annotations altogether.



fn find_first_string(values: &str)->&str{
    let bytes = values.as_bytes();
    
    for (index, &element) in bytes.iter().enumerate() {
        if element == b' '{
         return &values[0..=index];
        }
    }

    &values[..]

}

fn main(){
    let sentence = "Rust is cool";

    println!("{}", find_first_string(sentence));


}