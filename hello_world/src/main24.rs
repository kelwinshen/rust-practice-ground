//Generic in enum type
enum Result<T,E>{
    Ok(T),
    Err(E)
}

fn main(){
    let success: Result<&str, &str> =  Result::Ok("The transaction is success!");
    let failed: Result<&str, &str> = Result::Err("The transaction is failed!");

    match success {
        Result::Ok(value)=> println!("{}", value),
        Result::Err(error)=> println!("{}", error)
    }
}