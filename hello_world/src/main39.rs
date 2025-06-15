//Let's make some customable error
use::std::fmt;

struct TransactionError{
    details: String
}

impl TransactionError{
    fn new(msg: &str)-> TransactionError{
        TransactionError{
            details: String::from(msg)
        }
    }
}

impl fmt::Display for TransactionError{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f, "{}", self.details)
    }
}


fn do_something(value:i32)->Result<(), TransactionError>{
    if value <= 0 {
        return Err(TransactionError::new("value of the transaction is not right!"));
    } else{
        return Ok(());
    }

}

fn main(){
   match do_something(1){
    Ok(()) => println!("Transaction is successful!"),
    Err(error) => println!("{}", error)
   }
}